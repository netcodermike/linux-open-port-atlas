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

## Observed CI result

Passed: [target-platform build/check](https://github.com/netcodermike/linux-open-port-atlas/actions/runs/42204034421). Source commit: `70b1c2e9af834b8f1f0a8d4998472edc27ba4371`. Later documentation-only changes do not alter the tested implementation. The reproducible commands above identify test scope; native GUI/audio/permissions are not implied by compilation.
