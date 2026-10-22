# Contributing to Open Device Partnership

The Open Device Partnership project welcomes your suggestions and contributions! Before opening your first issue or pull request, please review our
[Code of Conduct](CODE_OF_CONDUCT.md) to understand how our community interacts in an inclusive and respectful manner.

## Contribution Licensing

Most of our code is distributed under the terms of the [MIT license](LICENSE), and when you contribute code that you wrote to our repositories,
you agree that you are contributing under those same terms. In addition, by submitting your contributions you are indicating that
you have the right to submit those contributions under those terms.

## Other Contribution Information

If you wish to contribute code or documentation authored by others, or using the terms of any other license, please indicate that clearly in your
pull request so that the project team can discuss the situation with you.

## Commit Message

* Use meaningful commit messages. See [this blogpost](http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html)

## PR Etiquette

* Create a draft PR first
* Make sure that your branch has `.github` folder and all the code linting/sanity check workflows are passing in your draft PR before sending it out to code reviewers.

## Clean Commit History

We disabled squashing of commit and would like to maintain a clean commit history. So please reorganize your commits with the following items:

* Each commit builds successfully without warning
* Miscellaneous commits to fix typos + formatting are squashed

## Regressions

When reporting a regression, please ensure that you use `git bisect` to find the first offending commit, as that will help us finding the culprit a lot faster.

## API compatibility advisory

[API compatibility advisory](.github/workflows/semver-advisory.yml) runs
`cargo-semver-checks` on feature PRs, comparing the PR head with its merge
base with the target branch. It permits additive APIs but reports breaking
APIs even when the PR already changes the package version. All crate
features are checked. Release-plz PRs (source branches beginning with
`release-plz-`, the default prefix) skip both analysis and reporting.

Detected breaks produce a warning, a downloadable `semver-report-<attempt>` artifact,
and an `api-breaking-change` label with a bot-updated PR comment. A later
compatible result removes the label and updates an existing comment.
Analysis failures report **analysis unavailable**, not a breaking change or
a clean result, and do not clear an existing label. The label is managed by
automation; use a different label for manually recorded release decisions.

This is advisory, not a merge gate. **Do not make `API compatibility
(advisory)` a required status check.** A detected break does not fail the
analysis job; an infrastructure/build/analyzer error can fail this optional
job and should be investigated. No branch-protection bypass is needed for
intentional API changes. Review the report and explain intended API breaks
and migrations in the PR. The analyzer does not catch every semantic or
behavioral compatibility change; review remains necessary.

The [reporter](.github/workflows/semver-advisory-report.yml) runs separately
with label/comment permissions, so fork PR builds receive no write token.
It executes only default-branch reporter code, treats result artifacts as
untrusted data, and ignores results for outdated PR heads or target bases.
Artifacts are bound to the run and attempt so retries cannot reuse an older
successful result after a failed analysis. If a missing fork artifact leaves
the original target base unknown, the reporter retains an existing comment
rather than overwrite it with a potentially stale result.
The reporter workflow and scripts must be merged into the default branch
before automatic labeling can work; analysis can run from a feature PR.

Run the script regression tests locally with:

```sh
node --test .github/scripts/semver-advisory.test.cjs
```

`cargo-semver-checks` is pinned to **0.50.0** because the prototype classifies
its final stderr summary (there is no stable JSON report). When upgrading,
verify the classifier against real compatible, breaking, and failed builds.

## Releases

The [Release-plz workflow](.github/workflows/release-plz.yml) follows the
[Release-plz quickstart](https://release-plz.dev/docs/github/quickstart).
It runs on pushes to `main` in `OpenDevicePartnership/bq25773`, not in forks.
One job creates or updates a draft release PR with version and changelog
changes; a separate job publishes to crates.io and creates the Git tag and
GitHub release after the release PR is merged.

[release-plz.toml](release-plz.toml) sets `release_always = false`, so merging
the workflow setup or another non-release PR does not publish an unpublished
version already in [Cargo.toml](Cargo.toml). Release PRs start as drafts to
follow this repository's PR etiquette.

### One-time maintainer setup

Before merging the workflow setup:

1. In the upstream repository's **Settings > Actions > General > Workflow
   permissions**, enable **Allow GitHub Actions to create and approve pull
   requests**. Organization policy must also permit this.
2. In the [bq25773 settings on crates.io](https://crates.io/crates/bq25773/settings),
   configure [Trusted Publishing](https://crates.io/docs/trusted-publishing)
   with repository owner `OpenDevicePartnership`, repository name `bq25773`,
   and workflow filename `release-plz.yml`. Leave the environment unset;
   this workflow does not use a GitHub environment.

Release-plz performs the OIDC token exchange itself. Do not set
`CARGO_REGISTRY_TOKEN` or add a separate crates.io authentication action.
Only the publishing job has `id-token: write`. If the workflow file is
renamed, update the trusted publisher on crates.io too.

### Reviewing and merging a release

Let Release-plz prepare the version and changelog changes rather than
manually bumping versions or pushing release tags.

The workflow uses the built-in `GITHUB_TOKEN`, which
[does not trigger other workflows](https://release-plz.dev/docs/github/token).
A maintainer must close and reopen the release PR to trigger its CI checks.
Repeat this after any bot update so the checks cover the latest commit.
Review the version and changelog, wait for all required checks to pass,
then mark the PR ready for review and merge it through the normal review
process. The resulting push to `main` triggers publication.
