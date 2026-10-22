const assert = require('node:assert/strict');
const { test } = require('node:test');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { classify } = require('./semver-advisory.cjs');
const { readResult, report } = require('./semver-report.cjs');

test('classifies only completed compatibility analysis, not arbitrary command failures', () => {
  assert.equal(classify(0, '     Summary no semver update required\n'), 'compatible');
  assert.equal(classify(100, '     Summary semver requires new major version: 1 major and 0 minor checks failed\n'), 'breaking');
  assert.equal(classify(1, 'error: could not compile bq25773'), 'error');
  assert.equal(classify(null, ''), 'error');
  assert.equal(classify(0, ''), 'error');
  assert.equal(classify(1, '     Summary no semver update required\n'), 'error');
  assert.equal(classify(1, '     Summary semver requires new minor version: 0 major and 1 minor checks failed\n'), 'error');
  assert.equal(classify(101, '     Summary semver requires new major version: 2 major and 0 minor checks failed\n'), 'error');
});

function fixture(status = 'breaking') {
  const writes = [];
  const sha = 'a'.repeat(40);
  const base = 'b'.repeat(40);
  const run = {
    id: 100, run_attempt: 1, workflow_id: 10, event: 'pull_request',
    head_sha: sha, head_branch: 'feature', head_repository: { full_name: 'contributor/bq25773' },
    conclusion: 'success', pull_requests: [{ number: 37, base: { sha: base } }],
  };
  const pr = {
    number: 37, state: 'open', head: { sha, ref: 'feature', repo: { full_name: 'contributor/bq25773' } },
    base: { sha: base, repo: { full_name: 'OpenDevicePartnership/bq25773' } },
    labels: [],
  };
  const result = { status, head_sha: sha, base_sha: base, run_id: '100', run_attempt: '1' };
  const github = {
    paginate: async (method, args) => (await method(args)).data,
    rest: {
      actions: {
        getWorkflowRun: async () => ({ data: run }),
      },
      pulls: {
        get: async () => ({ data: structuredClone(pr) }),
        list: async () => ({ data: [pr] }),
      },
      issues: {
        listComments: async () => ({ data: [] }),
        createLabel: async (args) => writes.push(['createLabel', args]),
        addLabels: async (args) => writes.push(['addLabels', args]),
        removeLabel: async (args) => writes.push(['removeLabel', args]),
        createComment: async (args) => writes.push(['createComment', args]),
        updateComment: async (args) => writes.push(['updateComment', args]),
      },
    },
  };
  const context = {
    repo: { owner: 'OpenDevicePartnership', repo: 'bq25773' },
    serverUrl: 'https://github.com', payload: { workflow_run: structuredClone(run) },
  };
  const core = { info() {}, warning() {} };
  return { github, context, core, result, run, pr, writes };
}

test('breaking report adds the advisory label and links to the analyzed run', async () => {
  const f = fixture();
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['createLabel', 'addLabels', 'createComment']);
  assert.deepEqual(f.writes[1][1].labels, ['api-breaking-change']);
  assert.equal(f.writes[1][1].issue_number, 37);
  assert.match(f.writes[2][1].body, /API breaking changes detected/);
  assert.match(f.writes[2][1].body, /actions\/runs\/100\/attempts\/1/);
});

test('compatible analysis removes only the advisory label without posting a new comment', async () => {
  const f = fixture('compatible');
  f.pr.labels = [{ name: 'api-breaking-change' }, { name: 'bug' }];
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['removeLabel']);
  assert.equal(f.writes[0][1].name, 'api-breaking-change');
});

test('analysis errors preserve an existing breaking label and say analysis is unavailable', async () => {
  const f = fixture('error');
  f.pr.labels = [{ name: 'api-breaking-change' }];
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['createComment']);
  assert.match(f.writes[0][1].body, /analysis unavailable/i);
});

test('updates only its own bot-authored report comment', async () => {
  const f = fixture('compatible');
  f.github.rest.issues.listComments = async () => ({ data: [
    { id: 1, user: { login: 'someone' }, body: '<!-- semver-advisory -->' },
    { id: 2, user: { login: 'github-actions[bot]' }, body: '<!-- semver-advisory -->' },
  ] });
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['updateComment']);
  assert.equal(f.writes[0][1].comment_id, 2);
  assert.match(f.writes[0][1].body, /No API breaking changes detected/);
});

for (const [name, mutate] of [
  ['new PR head', (f) => { f.pr.head.sha = 'c'.repeat(40); }],
  ['changed target baseline', (f) => { f.pr.base.sha = 'c'.repeat(40); }],
  ['closed PR', (f) => { f.pr.state = 'closed'; }],
  ['different fork', (f) => { f.pr.head.repo.full_name = 'attacker/bq25773'; }],
  ['different target repository', (f) => { f.pr.base.repo.full_name = 'other/repository'; }],
  ['superseded run attempt', (f) => { f.run.run_attempt = 2; }],
  ['non-PR run', (f) => { f.run.event = 'push'; }],
  ['cancelled run', (f) => { f.run.conclusion = 'cancelled'; }],
  ['release-plz automated PR', (f) => { f.run.head_branch = 'release-plz-2026-10-22'; f.pr.head.ref = f.run.head_branch; }],
  ['forged artifact head', (f) => { f.result.head_sha = 'c'.repeat(40); }],
  ['unknown artifact status', (f) => { f.result.status = 'approved'; }],
  ['artifact from a different run', (f) => { f.result.run_id = '99'; }],
  ['artifact from an older attempt', (f) => { f.result.run_attempt = '0'; }],
]) {
  test(`does not mutate GitHub for ${name}`, async () => {
    const f = fixture();
    mutate(f);
    await report(f);
    assert.deepEqual(f.writes, []);
  });
}

test('resolves fork PRs even when workflow_run.pull_requests is empty', async () => {
  const f = fixture();
  f.run.pull_requests = [];
  f.context.payload.workflow_run.pull_requests = [];
  await report(f);
  assert.ok(f.writes.some(([op]) => op === 'addLabels'));
});

test('finds fork PRs by source owner and branch when commit associations are unavailable', async () => {
  const f = fixture();
  f.run.pull_requests = [];
  let lookup;
  f.github.rest.pulls.list = async (args) => {
    lookup = args;
    return { data: [f.pr] };
  };
  await report(f);
  assert.ok(f.writes.some(([op]) => op === 'addLabels'));
  assert.equal(lookup.head, 'contributor:feature');
  assert.equal(lookup.state, 'open');
});

test('missing result reports unavailable without clearing labels', async () => {
  const f = fixture();
  f.result = null;
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['createComment']);
  assert.match(f.writes[0][1].body, /analysis unavailable/i);
});

test('does not publish when the PR changes while reading comments', async () => {
  const f = fixture();
  f.github.rest.issues.listComments = async () => {
    f.pr.head.sha = 'c'.repeat(40);
    return { data: [] };
  };
  await report(f);
  assert.deepEqual(f.writes, []);
});

test('a failed latest attempt cannot reuse a compatible result to clear a label', async () => {
  const f = fixture('compatible');
  f.run.conclusion = 'failure';
  f.pr.labels = [{ name: 'api-breaking-change' }];
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['createComment']);
  assert.match(f.writes[0][1].body, /analysis unavailable/i);
});

test('a timeout reports unavailable without clearing a label', async () => {
  const f = fixture();
  f.run.conclusion = 'timed_out';
  f.result = null;
  f.pr.labels = [{ name: 'api-breaking-change' }];
  await report(f);
  assert.deepEqual(f.writes.map(([op]) => op), ['createComment']);
  assert.match(f.writes[0][1].body, /analysis unavailable/i);
});

test('missing artifact from an outdated target base does not overwrite a current report', async () => {
  const f = fixture();
  f.result = null;
  f.pr.base.sha = 'c'.repeat(40);
  await report(f);
  assert.deepEqual(f.writes, []);
});

test('missing fork artifact with no original base identity cannot replace an existing report', async () => {
  const f = fixture();
  f.result = null;
  f.run.pull_requests = [];
  f.github.rest.issues.listComments = async () => ({ data: [
    { id: 2, user: { login: 'github-actions[bot]' }, body: '<!-- semver-advisory -->' },
  ] });
  await report(f);
  assert.deepEqual(f.writes, []);
});

test('reads only bounded JSON files and treats missing or malformed artifacts as unavailable', () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'semver-result-'));
  const file = path.join(directory, 'result.json');
  try {
    assert.equal(readResult(file), null);
    fs.writeFileSync(file, '{');
    assert.equal(readResult(file), null);
    fs.writeFileSync(file, ' '.repeat(4097));
    assert.equal(readResult(file), null);
    assert.equal(readResult(directory), null);
    fs.writeFileSync(file, '{"status":"error"}');
    assert.deepEqual(readResult(file), { status: 'error' });
  } finally {
    fs.rmSync(directory, { recursive: true });
  }
});
