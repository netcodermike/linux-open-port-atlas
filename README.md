# Linux Open Port Atlas

Map local TCP listeners and bound UDP sockets to addresses, ports, UIDs, and visible owning processes.

[![Build and verify](https://github.com/netcodermike/linux-open-port-atlas/actions/workflows/verify.yml/badge.svg)](https://github.com/netcodermike/linux-open-port-atlas/actions/workflows/verify.yml)

**Language:** Rust · **Platform:** Linux · **App type:** Linux network socket inspector

First-release learning implementation, with explicit scope below. This is one independently runnable app, not a placeholder in a combined repository.

## Features

- IPv4 and IPv6 TCP listener tables.
- Bound unconnected UDP endpoints, labeled separately by protocol.
- Socket inode-to-process mapping using visible /proc/PID/fd links.
- Port filter, deterministic sorting, and sanitized process names.

## Requirements

Rust stable (edition 2021), readable procfs, and a terminal. No crates, packet capture, active scans, or root privileges are required. Owner visibility depends on procfs permissions.

## Quick start

```sh
git clone https://github.com/netcodermike/linux-open-port-atlas.git
cd linux-open-port-atlas
cargo run --release
cargo run --release -- --port 22
```

Commands run from the repository root. Lines marked for a particular operating system apply only there. Language tools are prerequisites, not installed by the app. Close desktop windows normally; terminal/server apps stop with Ctrl+C unless their usage notes say otherwise.

## Example workflow

Start a harmless local server in another terminal, note its port, and run --port PORT. Compare the endpoint with ss -lntup. If process ownership is hidden by permissions, the endpoint remains visible with unknown/inaccessible ownership.

## Architecture, configuration, and data

`src/main.rs` parses /proc/net/tcp, tcp6, udp, and udp6. TCP state 0A means LISTEN; UDP state 07 is an unconnected bound socket. It decodes the kernel’s host-word address representation and scans visible process FD symlinks for socket inode matches. A socket can have several owners. The default view is the current network namespace and a non-atomic snapshot. --port accepts a numeric 0–65535 filter. There is no persistent store or network traffic generated. Tests cover IPv4/IPv6 decoding, state filtering, UID, and inode extraction.

## Verification

Build with the quick-start command where needed, then run:

```sh
cargo test
```

Manual acceptance: Compare a known local TCP listener and UDP-bound socket with ss, test port filtering, and run without elevated privileges to inspect owner visibility.

See [validation record](docs/VALIDATION.md) for observed checks and their limits. CI results verify only what the workflow runs; they do not establish native GUI correctness or production readiness.

## Limitations and next steps

Not an external port scanner, firewall verifier, packet sniffer, or complete socket inventory. Established TCP and connected UDP sessions are excluded. /proc tables are namespace-scoped and may hide other containers. Processes can exit between scans, and permissions may hide owners. System-wide FD scanning can be slow on busy hosts. The supplied byte-order fixtures target common little-endian Linux environments.

## Troubleshooting

Non-Linux systems exit with an explanation. Empty output may mean there are no matching listeners in this namespace. Restricted procfs can hide owners or tables; do not infer that unknown means unowned. Missing optional IPv6 tables are skipped; no readable socket tables is an error.

## Source map

- `Cargo.toml`
- `src/main.rs`

## Development and license

Read [contributing](CONTRIBUTING.md) before changing the app, and [security/privacy](SECURITY.md) before supplying private data. Use synthetic fixtures for tests. No telemetry, paid API key, or account is required by the app itself. Dependencies, when present, are stated in Requirements.

[MIT license](LICENSE), copyright 2026 netcodermike.
