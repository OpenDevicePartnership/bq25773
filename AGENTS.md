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
- MSRV: **Rust 1.94** (required by device-driver 2.1; see `Cargo.toml` and the `msrv` job in
  `.github/workflows/check.yml`).
- License: **MIT** (`LICENSE`).
- I2C device address: `0x6B` (`BQ_ADDR` in `src/lib.rs`).
- Library builds are `no_std`; unit-test builds enable `std` via
  `#![cfg_attr(not(test), no_std)]` in `src/lib.rs`.
- Single optional feature: `defmt` — enables `defmt` 1 formatting
  support and the `defmt` features of `device-driver` and
  `embedded-batteries-async`.
- Datasheet: <https://www.ti.com/lit/ds/symlink/bq25773.pdf>.

## Repository layout

```
.
├── AGENTS.md                       # ← this file
├── Cargo.toml                      # crate manifest, lints, features
├── Cargo.lock                      # checked in; update with dependency changes
├── build.rs                        # rerun-if-changed for device.ddsl
├── device.ddsl                     # device-driver v2 register definitions (source of truth)
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
│   └── device.rs                   # GENERATED from device.ddsl; do not edit by hand
├── supply-chain/
│   ├── README.md                   # dependency-vetting process
│   ├── audits.toml                 # local cargo-vet audits
│   ├── config.toml                 # cargo-vet policy and trusted imports
│   └── imports.lock                # imported audit records
└── .github/
    ├── copilot-instructions.md     # AI commit-message rules (folded in below)
    └── workflows/
        ├── cargo-vet.yml           # checks dependency audit coverage
        ├── cargo-vet-pr-comment.yml # reports cargo-vet results on PRs
        ├── check.yml               # fmt / clippy / semver / doc / hack / deny / test / msrv
        ├── device-driver.yml       # verifies src/device.rs matches device.ddsl
        └── nostd.yml               # cross-check for thumbv8m.main-none-eabihf
```

### `src/device.rs` is generated

`src/device.rs` is produced from `device.ddsl` by `device-driver-cli`
(`ddc`) and **must be regenerated** rather than hand-edited. The
committed file's header records generator version **2.1.1**.
`build.rs` only emits `rerun-if-changed`; ordinary Cargo builds use
the committed file and do not regenerate it.

The `device-driver-pregen-check` workflow
(`.github/workflows/device-driver.yml`) delegates to
[`tullom/device-driver-pregen-check@v1.1.0`](https://github.com/tullom/device-driver-pregen-check/blob/v1.1.0/action.yml).
Without overrides, that action installs the newest compatible CLI
matching `^2.1.1` and uses **Rust 1.94.0** with its stable `rustfmt`.
It fails if the regenerated, formatted file differs from the
committed one. Check the workflow and action defaults when updating
the generator.

To regenerate locally:

```sh
rustup toolchain install 1.94.0 --profile minimal --component rustfmt
cargo +1.94.0 install device-driver-cli --version '^2.1.1' --locked
ddc build --source device.ddsl --output src/device.rs rust --rust-defmt-feature=defmt
rustup run 1.94.0 rustfmt --edition 2024 --config newline_style=Unix src/device.rs
```

The pregeneration check uses **stable 1.94.0**, not nightly, and
forces Unix line endings. This is separate from the repository-wide
**nightly** format check below; changes must satisfy both checks.

## Building and testing

Commands below correspond to `.github/workflows/check.yml`,
`device-driver.yml`, `nostd.yml`, and `cargo-vet.yml`. CI runs on
Ubuntu with stable Rust unless otherwise noted. Shell environment
assignments below use POSIX syntax.

| Purpose | Command | CI job |
|---------|---------|--------|
| Format check (nightly) | `cargo +nightly fmt --check` | `fmt` |
| Clippy (lib) | `cargo clippy -- -Dwarnings` | `clippy` |
| Clippy (tests, default features) | `cargo clippy --tests -- -Dwarnings` | `test` |
| Clippy (tests, all features) | `cargo clippy --tests --all-features -- -Dwarnings` | `test` |
| Unit tests (default features) | `cargo test` | `test` |
| Unit tests (all features) | `cargo test --all-features` | `test` |
| Docs | `RUSTDOCFLAGS="--cfg docsrs" cargo doc --no-deps --all-features` | `doc` |
| MSRV build | `cargo +1.94 check` | `msrv` |
| `no_std` cross build | `cargo check --target thumbv8m.main-none-eabihf --no-default-features` | `nostd` |
| Feature powerset | `cargo hack --feature-powerset check` | `hack` |
| License/advisory scan | `cargo deny --all-features check` | `deny` |
| Semver check | `cargo semver-checks` | `semver` |
| Dependency audit coverage | `cargo vet --locked` | `vet` in `cargo-vet.yml` |
| Device manifest check | regenerate as above, then `git diff --exit-code -- src/device.rs` | `device-driver-pregen-check` |

Install `cargo-hack`, `cargo-deny`, and `cargo-semver-checks` with
`cargo install` if needed. The vet workflow uses `cargo-vet` **0.10.2**
(`cargo install cargo-vet --version 0.10.2`). Follow
[supply-chain/README.md](supply-chain/README.md) when dependency
changes require new audits or updated imports.

The cross build requires
`rustup target add thumbv8m.main-none-eabihf`. The format check
requires nightly with the `rustfmt` component.
For PowerShell, the docs command is
`$env:RUSTDOCFLAGS="--cfg docsrs"; cargo doc --no-deps --all-features`.

### Formatting (`rustfmt.toml`)

`rustfmt.toml` sets `group_imports = "StdExternalCrate"` and
`imports_granularity = "Module"`, which require **nightly**.
`max_width = 120` also applies on stable. Stable `rustfmt` warns
about and ignores the two unstable import-formatting options.
The `fmt` job uses `dtolnay/rust-toolchain@nightly`, so use nightly
for the repository-wide format check. Use the stable formatter
specified above when regenerating `src/device.rs`.

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
  (`mod device;` in `src/lib.rs`).
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
defmt = [
    "dep:defmt",
    "device-driver/defmt",
    "embedded-batteries-async/defmt",
]
```

Features must remain **additive**; enabling `defmt` must not change
behaviour beyond formatting/logging support. `cargo hack
--feature-powerset check` verifies that feature combinations
compile, not that runtime behaviour is unchanged. The `test` job
runs unit tests and test Clippy with both default and all features.

## Code conventions

- **Edition 2024**, MSRV 1.94. Do not use language features newer than
  what 1.94 understands; the `msrv` CI job runs `cargo check` on
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
  Do not hand-roll I2C transactions in new code — extend `device.ddsl`
  and regenerate instead.

## Driver-specific notes

- I2C address `0x6B` is fixed (`BQ_ADDR` in `src/lib.rs`).
- Control, status, ADC, and tuning registers in `device.ddsl` use
  **2-byte little-endian fieldsets**. Previously split low/high-byte
  registers are combined, for example `CHARGE_OPTION_0` at `0x00`,
  `CHARGE_PROFILE` at `0x10`, and `CHARGE_OPTION_2` at `0x32`.
  Access the full fieldset through the generated register API; do
  not reintroduce separate low/high-byte accessors.
- The current exceptions are `MANUFACTURE_ID` (`0x2E`) and
  `DEVICE_ID` (`0x2F`), which remain **1-byte fieldsets**.
  `read_chip_id` tests a one-byte manufacturer-ID read. Do not
  describe the current map as exclusively two-byte accesses.
- `AsyncRegisterInterface::write_register` sends one address byte
  followed by exactly `data.len()` payload bytes. Keep its
  bounds-checked buffer slicing; transfer lengths must follow the
  DDSL fieldset sizes, not a hard-coded assumption about all
  registers having the same width.
- `LARGEST_REG_SIZE_BYTES = 2`; the on-stack buffer in
  `write_register` is sized `1 + LARGEST_REG_SIZE_BYTES`. If you add a
  wider register to `device.ddsl`, bump this constant or the bounds
  check will start returning `RegisterSizeError`.
- The `device.ddsl` device block pins `register-address-type: u8`
  and `default-byte-order: LE`; v2 bit numbering is always LSB0.
  Code generation uses `--rust-defmt-feature=defmt`. New registers
  should inherit these defaults; deviate only with a clear datasheet reference.
- Tests use `embedded-hal-mock`'s `eh1::i2c::{Mock, Transaction}` and
  the `tokio` runtime (`#[tokio::test]`). Always call
  `i2c.done()` at the end of a test to assert all expected
  transactions ran. See `src/lib.rs::tests` for the pattern.
  `disable_external_ilim_pin` demonstrates a two-byte
  read/modify/write that preserves the other fields.
- Preserve bus encodings when changing field types or combining
  registers. One-bit enum selections with values 0/1 must remain
  unsigned; a one-bit signed field cannot represent 1.

## Commit and PR conventions

From `.github/copilot-instructions.md` and `CONTRIBUTING.md`:

- **Subject line**: capitalized, ≤ 50 characters, imperative mood
  (e.g. `Fix bug`, not `Fixed bug`).
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

- Do not hand-edit `src/device.rs`. Regenerate from `device.ddsl` (see
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
- Do not break feature additivity: enabling `defmt` must not change
  observable behaviour beyond formatting/logging support.
  Feature-powerset compilation alone does not prove this.
- Do not add `std`-only dependencies to `[dependencies]`. Anything
  std-flavoured belongs in `[dev-dependencies]` (today: `tokio`,
  `embedded-hal-mock`).
- Do not add `Signed-off-by` from an AI session.
- Do not force-push shared branches; do not rewrite history on `main`.
- Do not commit secrets or vendor-confidential datasheets.
- Do not treat `include = […]` in `Cargo.toml` as a Git allowlist.
  It controls crate packaging; update it intentionally when adding
  files needed by the published crate. Repository-only documentation,
  workflows, and supply-chain audit files need not be packaged.

## How to find more context

- Datasheet: <https://www.ti.com/lit/ds/symlink/bq25773.pdf>.
- `device-driver` crate (DSL + codegen):
  <https://docs.rs/device-driver>.
- `embedded-hal-async`: <https://docs.rs/embedded-hal-async>.
- `embedded-batteries-async`: <https://docs.rs/embedded-batteries-async>.
- `embedded-hal-mock` (used in tests): <https://docs.rs/embedded-hal-mock>.
- Sibling driver crates under
  <https://github.com/OpenDevicePartnership> use pregenerated drivers
  and similar CI scaffolding; older crates may still use v1 YAML.
  Consult them for conventions not covered here, but use v2 DDSL in this crate.
- Upstream issue tracker and PRs:
  <https://github.com/OpenDevicePartnership/bq25773>.

## Incorporated from `.github/copilot-instructions.md`

The commit-message and AI-attribution rules from
`.github/copilot-instructions.md` are reproduced below. That file
points here for general project guidance. If the shared rules ever
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
