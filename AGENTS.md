# AGENTS.md

Guidance for AI coding assistants (and humans) working in this repository.
This file is the canonical source of project conventions, build/test
commands, and workflow rules. Keep it in sync with CI; if a command here
disagrees with `.github/workflows/`, the workflows win and this file
must be updated.

## What this crate is

`bq25773` is a `#[no_std]`, platform-agnostic Rust driver for the
Texas Instruments **BQ25773** buck-boost battery charge controller
(2- to 5-cell). It is built on the `embedded-hal` 1.0 traits and uses
the **async** I2C trait from `embedded-hal-async`. The high-level
charger API is implemented against the
[`embedded-batteries-async`](https://crates.io/crates/embedded-batteries-async)
`charger::Charger` trait.

Key facts:

- Crate name / version: `bq25773` (see `Cargo.toml`).
- Edition: **2024**.
- MSRV: **Rust 1.85** (see `Cargo.toml` and the `msrv` job in
  `.github/workflows/check.yml`).
- License: **MIT** (`LICENSE`).
- I2C device address: `0x6B` (`BQ_ADDR` in `src/lib.rs`).
- `no_std` always; `std` is only enabled under `cfg(test)`
  (`#![cfg_attr(not(test), no_std)]` in `src/lib.rs`).
- Single optional feature: `defmt-03` — enables `defmt` 0.3 logging
  through `device-driver` and `embedded-batteries-async`.
- Datasheet: <https://www.ti.com/lit/ds/symlink/bq25773.pdf>.

## Repository layout

```
.
├── AGENTS.md                       # ← this file
├── Cargo.toml                      # crate manifest, lints, features
├── Cargo.lock                      # checked in (library + binary policy)
├── build.rs                        # rerun-if-changed for device.yaml
├── device.yaml                     # device-driver register manifest (source of truth)
├── rustfmt.toml                    # nightly-only options (see "Formatting")
├── deny.toml                       # cargo-deny configuration
├── CONTRIBUTING.md                 # contributor rules (commit style, PR etiquette)
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── CODEOWNERS
├── README.md
├── LICENSE                         # MIT
├── src/
│   ├── lib.rs                      # public API: Bq25773, DeviceInterface, error type, tests
│   └── device.rs                   # GENERATED from device.yaml; do not edit by hand
└── .github/
    ├── copilot-instructions.md     # AI commit-message rules (folded in below)
    └── workflows/
        ├── check.yml               # fmt / clippy / semver / doc / hack / deny / test / msrv
        ├── device-driver.yml       # verifies src/device.rs matches device.yaml
        └── nostd.yml               # cross-check for thumbv8m.main-none-eabihf
```

### `src/device.rs` is generated

`src/device.rs` is produced from `device.yaml` by `device-driver-cli`
and **must be regenerated** rather than hand-edited. The
`device-driver-pregen-check` workflow
(`.github/workflows/device-driver.yml`) fails the build if the
committed file does not match a freshly generated one.

To regenerate locally:

```powershell
cargo install device-driver-cli                                  # once
device-driver-cli --manifest device.yaml --device-name Device -o src/device.rs
rustup run nightly rustfmt --edition 2024 src/device.rs
```

The CI job formats the regenerated file with **nightly** `rustfmt`
before diffing, because `rustfmt.toml` contains unstable options. Use
the same nightly run locally or the diff will perpetually fail.

## Building and testing

All commands below mirror `.github/workflows/check.yml`,
`device-driver.yml`, and `nostd.yml`. They have been verified on a
Windows + stable-toolchain checkout of this repo.

| Purpose                | Command                                                                  | CI job                |
|------------------------|--------------------------------------------------------------------------|-----------------------|
| Format check (nightly) | `cargo +nightly fmt --check`                                             | `fmt`                 |
| Clippy (lib)           | `cargo clippy -- -Dwarnings`                                             | `clippy`              |
| Clippy (tests)         | `cargo clippy --tests -- -Dwarnings`                                     | `test` (second step)  |
| Unit tests             | `cargo test`                                                             | `test`                |
| Docs                   | `$env:RUSTDOCFLAGS="--cfg docsrs"; cargo doc --no-deps --all-features`   | `doc`                 |
| MSRV build             | `cargo check`            (under Rust 1.85)                               | `msrv`                |
| `no_std` cross build   | `cargo check --target thumbv8m.main-none-eabihf --no-default-features`   | `nostd`               |
| Feature powerset       | `cargo hack --feature-powerset check`  (requires `cargo install cargo-hack`) | `hack`            |
| License/advisory scan  | `cargo deny --all-features check`      (requires `cargo install cargo-deny`) | `deny`            |
| Semver check           | `cargo semver-checks`                  (requires `cargo install cargo-semver-checks`) | `semver` |
| Device manifest check  | see "Regenerating `src/device.rs`" above                                 | `device-driver-pregen-check` |

### Formatting (`rustfmt.toml`)

`rustfmt.toml` enables unstable options (`group_imports =
"StdExternalCrate"`, `imports_granularity = "Module"`,
`max_width = 120`). These work only on the **nightly** channel; on
stable, `cargo fmt --check` will print warnings like
`Warning: can't set `imports_granularity = Module`, unstable features
are only available in nightly channel` and silently ignore those
options. The `fmt` job uses `dtolnay/rust-toolchain@nightly`. Always
use nightly for fmt to match CI.

### Clippy policy

Lint configuration is in `Cargo.toml` under `[lints.clippy]` /
`[lints.rust]`. The crate **denies** (not warns):

- `unsafe_code`, `missing_docs` (rust lints).
- `correctness`, `pedantic`, `perf`, `style`, `suspicious`.
- `indexing_slicing`, `panic`, `panic_in_result_fn`, `todo`,
  `unimplemented`, `unreachable`, `unwrap_used`.

Practical consequences:

- No `unsafe` blocks in hand-written code. The generated `src/device.rs`
  has its own `#[allow(unsafe_code)]` scoped to that module
  (`mod device { … }` in `src/lib.rs`).
- No `.unwrap()`, `.expect()`-into-panic, `panic!`, `todo!`,
  `unimplemented!`, or `unreachable!` outside `#[cfg(test)]`. Tests opt
  in with `#![allow(clippy::unwrap_used)]` (see
  `src/lib.rs::tests`).
- No raw `slice[i]` / `slice[a..b]` indexing — use `.get()` /
  `.get_mut()` and propagate the error. The existing `write_register`
  in `src/lib.rs` is the reference pattern: it bounds-checks the
  buffer via `.get_mut(..)` / `.get(..)` and returns
  `BQ25773Error::RegisterSizeError`.
- Public items must have doc comments (`missing_docs = "deny"`).
  `src/lib.rs` currently relaxes this with `#![allow(missing_docs)]`;
  do not rely on that — write doc comments for any new public API.

### Features

Only one feature exists:

```toml
defmt-03 = [
    "dep:defmt",
    "device-driver/defmt-03",
    "embedded-batteries-async/defmt",
]
```

Features must remain **additive**; enabling `defmt-03` must not change
behaviour beyond logging. CI enforces this through `cargo hack
--feature-powerset check`.

## Code conventions

- **Edition 2024**, MSRV 1.85. Do not use language features newer than
  what 1.85 understands; the `msrv` CI job runs `cargo check` on
  exactly that version.
- **Async-only I2C**. The driver targets `embedded-hal-async`. Do not
  add blocking-only paths unless you also keep an async path; the
  `charger::Charger` impl is `async` end-to-end.
- **Error type**: `BQ25773Error<I2cError>` with two variants
  (`Bus(I2cError)`, `RegisterSizeError`). New error conditions should
  become new variants on this enum, not panics, and must be mapped in
  `charger::Error::kind` (see `src/lib.rs`).
- **Imports**: nightly rustfmt groups them as `StdExternalCrate` and
  collapses by module (`imports_granularity = "Module"`). Run nightly
  `cargo fmt` before committing.
- **Line width**: 120 columns (`rustfmt.toml`).
- **No `unsafe`** in `src/lib.rs`. The generated `src/device.rs` is the
  only place `unsafe_code` is allowed, and only via the scoped
  `#[allow(unsafe_code)]` on `mod device`.
- **Register access** must go through the generated `device-driver`
  API (`self.device.<register>().read_async()/write_async()/modify_async()`).
  Do not hand-roll I2C transactions in new code — extend `device.yaml`
  and regenerate instead.

## Driver-specific notes

- I2C address `0x6B` is fixed (`BQ_ADDR` in `src/lib.rs`).
- The BQ25773 mixes 1-byte and 2-byte registers. The
  `AsyncRegisterInterface::write_register` impl in `src/lib.rs`
  intentionally slices the outgoing buffer to `..=data.len()` so a
  1-byte write never spills into the next register. Preserve this
  invariant.
- `LARGEST_REG_SIZE_BYTES = 2`; the on-stack buffer in
  `write_register` is sized `1 + LARGEST_REG_SIZE_BYTES`. If you add a
  wider register to `device.yaml`, bump this constant or the bounds
  check will start returning `RegisterSizeError`.
- The `device.yaml` config block pins `register_address_type: u8`,
  `default_byte_order: LE`, `default_bit_order: LSB0`, and
  `defmt_feature: defmt-03`. New registers should inherit these
  defaults; deviate only with a clear datasheet reference.
- Tests use `embedded-hal-mock`'s `eh1::i2c::{Mock, Transaction}` and
  the `tokio` runtime (`#[tokio::test]`). Always call
  `i2c.done()` at the end of a test to assert all expected
  transactions ran. See the three tests in `src/lib.rs::tests` for the
  pattern.

## Commit and PR conventions

From `.github/copilot-instructions.md` and `CONTRIBUTING.md`, verified
against `git log --pretty=%s` on `main`:

- **Subject line**: capitalized, ≤ 50 characters, imperative mood
  (e.g. `Fix bug`, not `Fixed bug`). Existing history confirms this
  (`Bump device-driver to 1.0.9`, `Pregenerate device-driver manifest
  file`, `Remove panic path in write_register(), …`).
- Blank line between subject and body; wrap body at 72 columns;
  explain *what* and *why*, not *how*.
- **Clean history**: squash-merging is disabled (`CONTRIBUTING.md`).
  Each commit must build without warnings; squash typo/format fixups
  into the commit that introduced them.
- **PR etiquette**: open as **draft** first; make sure all
  `.github/workflows/` checks pass on the draft before requesting
  review.
- **AI attribution** (mandatory for AI-assisted commits):
  ```
  Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
  ```
  Examples: `Assisted-by: GitHub Copilot:claude-opus-4.7`. Verify the
  agent name and model version each session — do not copy from
  history. Basic tools (git, cargo, editors) are *not* listed.
- **AI agents MUST NOT** add `Signed-off-by`. Only humans can certify
  the DCO.
- **Regressions**: when reporting one, run `git bisect` to identify
  the first offending commit before filing.

## What not to do

- Do not hand-edit `src/device.rs`. Regenerate from `device.yaml` (see
  above) or `device-driver-pregen-check` will fail.
- Do not add `unsafe` to `src/lib.rs` (the crate denies `unsafe_code`).
- Do not introduce `.unwrap()`, `.expect("…")`-as-panic, `panic!`,
  `todo!`, `unimplemented!`, or `unreachable!` outside `#[cfg(test)]`.
- Do not index slices with `[i]` / `[a..b]`; use `.get()` / `.get_mut()`
  and return `BQ25773Error::RegisterSizeError` (or a new variant) on
  failure.
- Do not bump MSRV without updating both `Cargo.toml` reasoning and
  the `msrv` matrix in `.github/workflows/check.yml`, and without
  justifying the bump in the commit body.
- Do not break feature additivity: enabling `defmt-03` must not change
  observable behaviour beyond logging. `cargo hack --feature-powerset
  check` enforces this.
- Do not add `std`-only dependencies to `[dependencies]`. Anything
  std-flavoured belongs in `[dev-dependencies]` (today: `tokio`,
  `embedded-hal-mock`).
- Do not add `Signed-off-by` from an AI session.
- Do not force-push shared branches; do not rewrite history on `main`.
- Do not commit secrets, vendor-confidential datasheets, or any file
  outside the `include = […]` list in `Cargo.toml` without updating
  that list intentionally.

## How to find more context

- Datasheet: <https://www.ti.com/lit/ds/symlink/bq25773.pdf>.
- `device-driver` crate (DSL + codegen):
  <https://docs.rs/device-driver>.
- `embedded-hal-async`: <https://docs.rs/embedded-hal-async>.
- `embedded-batteries-async`: <https://docs.rs/embedded-batteries-async>.
- `embedded-hal-mock` (used in tests): <https://docs.rs/embedded-hal-mock>.
- Sibling driver crates under
  <https://github.com/OpenDevicePartnership> use the same `device.yaml`
  + `device-driver-cli` pattern and the same CI scaffolding; consult
  them for conventions not covered here.
- Issue tracker and PRs:
  <https://github.com/OpenDevicePartnership/bq25773>.

## Incorporated from `.github/copilot-instructions.md`

The following is the full content of
`.github/copilot-instructions.md` at the time this file was authored.
This `AGENTS.md` is a strict superset of that file; if the two ever
disagree, `AGENTS.md` wins and `copilot-instructions.md` should be
updated to match.

> # Copilot Instructions
>
> ## Commit Messages
> - Subject line: capitalized, 50 characters or less, imperative mood
>   (e.g., "Fix bug" not "Fixed bug")
> - Separate subject from body with a blank line
> - Wrap body text at 72 characters
> - Use the body to explain *what* and *why*, not *how*
>
> ## AI Attribution
> Every commit that includes AI-generated or AI-assisted work **must**
> contain an `Assisted-by` trailer in the commit message:
> ```
> Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
> ```
> Where:
> - `AGENT_NAME` is the name of the AI tool or framework (e.g.,
>   `GitHub Copilot`)
> - `MODEL_VERSION` is the specific model version used (e.g.,
>   `claude-opus-4.6`)
> - `[TOOL1] [TOOL2]` are optional specialized analysis tools used
>   (e.g., `coccinelle`, `sparse`, `smatch`, `clang-tidy`)
> Basic development tools (git, cargo, editors) should not be listed.
> AI agents **must** verify their own identity (agent name and model
> version) before composing the `Assisted-by` trailer — do not assume
> or hard-code a model name from a previous session.
> AI agents **MUST NOT** add `Signed-off-by` tags. Only humans can
> certify the Developer Certificate of Origin.

## Model selection & cost discipline

Premium models (Opus, GPT-5 family, "high"/"xhigh" reasoning variants)
cost an order of magnitude more than standard models (Sonnet, Haiku,
mini). Most steps in a typical task do not need premium reasoning,
and over-using premium models wastes credits without improving
outcomes. The rules below apply to *all* model selection: your own
session, sub-agents launched via the `task` tool, and parallel work
launched via `/fleet`.

### Default posture

- **Default to the cheapest model that can do the job.** Reach for a
  premium model only when one of the escalation triggers below is hit.
- **Plan with premium, execute with cheap.** Spend at most one or two
  premium turns on design / planning, then downshift to a cheaper
  model for mechanical execution of the plan.
- **Never bump the model "just in case."** If you cannot articulate
  *why* a cheaper model would fail, use the cheaper model.

### Escalation triggers (use a premium model)

Reach for a premium model when *any* of these are true:

- Cross-module refactor, architectural design, or API design from
  scratch.
- Subtle correctness reasoning: concurrency, lifetimes, `unsafe`,
  FFI ABI, cryptography, safety-critical control paths.
- Debugging a failure that survived one prior cheap-model attempt.
- Reviewing code on a safety-, security-, or money-critical path.
- The diff cannot be predicted in advance — i.e. there is genuine
  creative or design work to do, not just typing.

### De-escalation triggers (use a cheap model)

Use the cheapest available model when *any* of these are true:

- Searching, reading, summarising files or docs.
- Single-file mechanical edits: rename, format, lint fix, dependency
  bump, boilerplate, scaffolding from a known template.
- Generating tests for code that already works.
- Running builds, tests, linters, or other commands where the model
  only needs to report success/failure.
- Routine commits, PR descriptions, changelog entries.
- The diff is essentially predictable before generation.

### Sub-agent routing (the `task` tool)

When delegating with the `task` tool, set `model:` explicitly. Do not
let sub-agents inherit a premium default for cheap work.

| Sub-agent type    | Default model             | Override to                                     |
|-------------------|---------------------------|-------------------------------------------------|
| `explore`         | cheap                     | keep cheap (`claude-haiku-4.5` or `gpt-5-mini`) |
| `task` (run cmd)  | cheap                     | keep cheap                                      |
| `research`        | cheap for breadth         | premium only for the final synthesis            |
| `general-purpose` | match task                | cheap for mechanical work; premium for design   |
| `rubber-duck`     | premium                   | keep premium — this is where reasoning pays off |
| `code-review`     | premium on critical paths | cheap on cosmetic / mechanical diffs            |

### `/fleet` (parallel sub-agents) rules

- Fleet mode multiplies cost by the fleet width. Apply the rules
  above *per worker*, not in aggregate.
- Split a fleet job along complexity lines: route the cheap,
  parallelisable workers (file edits, test runs, doc updates) to a
  cheap model; reserve premium models for the small number of
  workers that need real reasoning.
- If every worker in a fleet would need a premium model, the work is
  probably not a good fit for fleet mode — reconsider the
  decomposition before paying N× premium.

### Session hygiene

- Keep sessions short and focused. Long premium sessions are the
  single largest source of waste because every turn re-processes the
  full history.
- Use `/compact` when the conversation grows long, and `/new` for
  unrelated work.
- Prefer `/ask` for one-off side questions so they don't extend the
  main session.

### When in doubt

Ask: *"If a cheaper model produced the wrong answer here, would I
catch it in seconds (compiler, tests, my own review) or in
weeks (production incident)?"* If the former, use the cheap model
and let the feedback loop do its job.
