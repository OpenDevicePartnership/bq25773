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
