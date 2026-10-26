# rust-cov

A composite GitHub Action that measures Rust line coverage with
[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) and gates it on the
**merged `DA` records** of the lcov report, so each source line is counted once.

```yaml
- name: Coverage
  uses: cpmech/rust-cov@v1
  with:
    args: --workspace --features umfpack
    ignore-filename-regex: 'build.rs|src/bin/'
    min-lines: 90
```

The caller installs the Rust toolchain; this action installs cargo-llvm-cov (and
`llvm-tools-preview` when rustup is present), runs the tests under instrumentation,
writes the lcov report, checks the threshold and optionally uploads the report.

## Why not `cargo llvm-cov --fail-under-lines`

llvm-cov's own summary counts a line once per monomorphization. A function that
dispatches on a const generic keeps every branch in every instantiation, so the
branches belonging to another dimension are unreachable in this one — they are a
different function — and the percentage ends up describing the instantiations
rather than the source. Measured on such a crate, the same run reads 90.60% on
llvm-cov's summary and 91.24% on the `DA` records; the difference decides whether a
95% gate is meaningful.

`LF:`/`LH:` are not usable either: cargo-llvm-cov writes them from the
instantiation-based summary, so they carry the same artifact. The `DA:` records are
already merged per source line, and `check-coverage.bash` counts the maximum hits
per line.

## Inputs

| input | default | meaning |
|---|---|---|
| `args` | `--workspace` | Extra `cargo llvm-cov` arguments. Split on whitespace, so no spaces inside an argument. |
| `ignore-filename-regex` | *(empty)* | Passed as `--ignore-filename-regex`. Use this rather than putting the flag in `args`. |
| `lcov-path` | `lcov.info` | Where the report is written. |
| `min-lines` | `0` | Minimum line coverage percentage. `0` reports without gating. |
| `version` | `latest` | cargo-llvm-cov version, e.g. `0.9.1`. Pin it for reproducibility. |
| `upload` | `false` | Upload the lcov report as an artifact. |
| `artifact-name` | `lcov` | Artifact name when `upload` is true. |

## Outputs

`covered`, `total` and `percentage` — the numbers the gate used, available as
`${{ steps.<id>.outputs.percentage }}`.

## Requirements

- A Rust toolchain installed by the caller (`dtolnay/rust-toolchain@stable`, a
  matrix leg, whatever the repository already uses).
- A prebuilt cargo-llvm-cov exists for Linux x86_64/aarch64 and macOS
  x86_64/arm64; on anything else install cargo-llvm-cov yourself and drop the
  `version` input (the action will then try to install it again, so prefer
  running `check-coverage.bash` directly there).
- `llvm-tools-preview` comes from rustup when available; without rustup, add it to
  the toolchain setup.

## Versioning

`v1` is a moving major tag. For a hard-pinned dependency use a commit SHA:

```yaml
  uses: cpmech/rust-cov@<sha>
```

## Licence

MIT.
