# Validation

Authoring date: November 6, 2026. Host: Windows. Target: Linux.

## Local result

Rust/Linux runtime unavailable on authoring host. CI builds/tests parsers and reads live procfs.

## Reproducible checks

The GitHub Actions workflow builds/runs the following on `ubuntu-latest`:

```sh
cargo test
cargo run --release
```

## Manual acceptance

Compare a known local TCP listener and UDP-bound socket with ss, test port filtering, and run without elevated privileges to inspect owner visibility.

A workflow file is not evidence of a passing run. Observed CI run links and source commits are recorded after publication. Native GUI, sound, sleep/resume, and platform permissions require direct target-platform acceptance even when model tests pass.
