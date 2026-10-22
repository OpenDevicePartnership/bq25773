const fs = require('node:fs');

const label = 'api-breaking-change';
const marker = '<!-- semver-advisory -->';

function readResult(file) {
  try {
    const stat = fs.lstatSync(file);
    if (!stat.isFile() || stat.size > 4096) return null;
    return JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch {
    return null;
  }
}

async function report({ github, context, core, result }) {
  const repo = context.repo;
  const triggered = context.payload.workflow_run;
  const { data: run } = await github.rest.actions.getWorkflowRun({ ...repo, run_id: triggered.id });
  if (run.event !== 'pull_request' || run.run_attempt !== triggered.run_attempt ||
      !['success', 'failure', 'timed_out', 'startup_failure'].includes(run.conclusion) ||
      run.head_branch.startsWith('release-plz-')) return;

  // Obtain PR numbers from GitHub, never from an untrusted artifact. Fork runs
  // can have neither pull_requests metadata nor a commit-to-PR association.
  const associated = run.pull_requests.length ? run.pull_requests : await github.paginate(
    github.rest.pulls.list,
    { ...repo, state: 'open', head: `${run.head_repository.full_name.split('/')[0]}:${run.head_branch}`, per_page: 100 },
  );
  for (const associatedPr of associated) {
    const { data: pr } = await github.rest.pulls.get({ ...repo, pull_number: associatedPr.number });
    if (pr.state !== 'open' || pr.head.sha !== run.head_sha ||
        pr.head.ref !== run.head_branch || pr.head.repo?.full_name !== run.head_repository?.full_name ||
        pr.base.repo.full_name !== `${repo.owner}/${repo.repo}`) continue;
    if (result && (result.head_sha !== pr.head.sha || result.base_sha !== pr.base.sha ||
        result.run_id !== String(run.id) || result.run_attempt !== String(run.run_attempt) ||
        !['compatible', 'breaking', 'error'].includes(result.status))) {
      core.warning('Ignoring stale or invalid semver result.');
      continue;
    }
    const originalBase = run.pull_requests.find((item) => item.number === pr.number)?.base?.sha;
    if (!result && originalBase && originalBase !== pr.base.sha) continue;
    const status = run.conclusion === 'success' ? result?.status || 'error' : 'error';
    const issue = { ...repo, issue_number: pr.number };
    const comments = await github.paginate(github.rest.issues.listComments, { ...issue, per_page: 100 });
    const existing = comments.find((comment) =>
      comment.user?.login === 'github-actions[bot]' && comment.body?.includes(marker));
    if (!result && !originalBase && existing) {
      core.warning('Cannot establish target-base freshness; retaining the existing report.');
      continue;
    }
    // Recheck after reads so an outdated run does not overwrite a newer head.
    const { data: latest } = await github.rest.pulls.get({ ...repo, pull_number: pr.number });
    if (latest.state !== 'open' || latest.head.sha !== pr.head.sha || latest.base.sha !== pr.base.sha) continue;

    if (status === 'breaking') {
      try {
        await github.rest.issues.createLabel({ ...repo, name: label, color: 'd93f0b',
          description: 'Automated advisory: API incompatibilities detected' });
      } catch (error) {
        if (error.status !== 422 || !error.response?.data?.errors?.some((item) => item.code === 'already_exists')) {
          throw error;
        }
      }
      await github.rest.issues.addLabels({ ...issue, labels: [label] });
    } else if (status === 'compatible' && pr.labels.some((item) => item.name === label)) {
      try {
        await github.rest.issues.removeLabel({ ...issue, name: label });
      } catch (error) {
        if (error.status !== 404) throw error;
      }
    }

    if (status === 'compatible' && !existing) continue;
    const titles = {
      breaking: '⚠️ API breaking changes detected',
      compatible: '✓ No API breaking changes detected',
      error: '⚠️ API compatibility analysis unavailable',
    };
    const url = `${context.serverUrl}/${repo.owner}/${repo.repo}/actions/runs/${run.id}/attempts/${run.run_attempt}`;
    const body = `${marker}\n## ${titles[status]}\n\n` +
      `Analyzed PR head: \`${pr.head.sha}\`.\n\n` +
      'This report is advisory and does not block merging. Review whether any API breaks are intentional. ' +
      'Release-plz independently checks the eventual release and selects its version bump.\n\n' +
      (status === 'error' ? 'No compatibility conclusion is available; any existing breaking-change label is retained.\n\n' : '') +
      `[View analysis and download the semver-report artifact](${url}).`;
    if (existing) {
      await github.rest.issues.updateComment({ ...repo, comment_id: existing.id, body });
    } else {
      await github.rest.issues.createComment({ ...issue, body });
    }
  }
}

module.exports = { readResult, report };
