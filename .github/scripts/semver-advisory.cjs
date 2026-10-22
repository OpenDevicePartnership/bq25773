const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

// cargo-semver-checks 0.50.0 has no stable JSON report. Match its final
// stderr summary, not its exit code alone. Keep the workflow version pinned.
function classify(exitCode, stderr) {
  if (exitCode === 0 && /^\s*Summary no semver update required\s*$/m.test(stderr)) {
    return 'compatible';
  }
  // v0.50.0 distinguishes completed lint failures (100) from tool errors (101).
  // https://github.com/obi1kenobi/cargo-semver-checks/blob/v0.50.0/src/main.rs
  if (exitCode === 100 && /^\s*Summary semver requires new major version: [1-9]\d* major and \d+ minor checks failed\s*$/m.test(stderr)) {
    return 'breaking';
  }
  return 'error';
}

function analyze() {
  const event = JSON.parse(fs.readFileSync(process.env.GITHUB_EVENT_PATH, 'utf8'));
  const head = event.pull_request.head.sha;
  const base = event.pull_request.base.sha;
  if (![head, base].every((sha) => /^[a-f0-9]{40}$/.test(sha))) {
    throw new Error('Invalid PR commit identity');
  }
  const directory = process.env.SEMVER_REPORT_DIR;
  fs.mkdirSync(directory, { recursive: true });
  const result = { status: 'error', head_sha: head, base_sha: base,
    run_id: process.env.GITHUB_RUN_ID, run_attempt: process.env.GITHUB_RUN_ATTEMPT };
  const baseline = spawnSync('git', ['merge-base', head, base], { encoding: 'utf8' });
  let log = baseline.stderr || '';
  if (baseline.status === 0 && /^[a-f0-9]{40}$/.test(baseline.stdout.trim())) {
    result.baseline_sha = baseline.stdout.trim();
    const check = spawnSync('cargo', [
      'semver-checks', '--baseline-rev', result.baseline_sha,
      // "minor" is the tool's compatibility class, not a numeric 0.x bump:
      // allow additive APIs, but detect breaking APIs even after a version bump.
      '--release-type', 'minor', '--all-features', '--color', 'never',
    ], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
    log = `${check.stdout || ''}\n${check.stderr || ''}\n${check.error || ''}`;
    result.status = check.error ? 'error' : classify(check.status, check.stderr || '');
  }
  fs.writeFileSync(path.join(directory, 'result.json'), JSON.stringify(result));
  fs.writeFileSync(path.join(directory, 'report.txt'), log);
  const messages = {
    compatible: 'No API breaking changes detected.',
    breaking: 'API breaking changes detected. Review the semver-report artifact; this is advisory.',
    error: 'API compatibility analysis unavailable. Review the semver-report artifact.',
  };
  const message = messages[result.status];
  // Do not echo analyzer output: PR-controlled build scripts can print Actions commands.
  console.log(result.status === 'compatible' ? message : `::warning::${message}`);
  if (process.env.GITHUB_STEP_SUMMARY) {
    fs.appendFileSync(process.env.GITHUB_STEP_SUMMARY,
      `## API compatibility advisory\n\n${message}\n\nBaseline: \`${result.baseline_sha || 'unavailable'}\`.\n`);
  }
  if (result.status === 'error') process.exitCode = 1;
}

if (require.main === module) analyze();
module.exports = { classify };
