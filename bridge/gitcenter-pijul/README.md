# gitcenter-pijul

A small Rust process that exposes a narrow, authenticated JSON-lines protocol to a trusted GitCenter server process. It is not a browser API and does not expose arbitrary command execution.

## Compatibility target

- Bridge protocol: **1**
- Pijul CLI: **1.0.0-beta.24**
- Pijul invocation: subprocess through Rust's std::process::Command, never through a shell.
- No direct libpijul dependency is claimed in this first bridge slice. The Pijul Rust crates have been reorganized; this bridge pins the CLI contract while keeping a future native-library adapter behind this process boundary.

Install the exact CLI version:

    cargo install pijul --version 1.0.0-beta.24 --locked

The bridge rejects other Pijul versions instead of silently accepting an unverified CLI.

## Build

    cargo build --release --manifest-path bridge/gitcenter-pijul/Cargo.toml
    cargo test --manifest-path bridge/gitcenter-pijul/Cargo.toml

## Runtime configuration

The process requires:

- GITCENTER_PIJUL_ROOT: existing canonical directory containing GitCenter-managed Pijul repositories.
- GITCENTER_PIJUL_TOKEN: at least 32 bytes of high-entropy secret material, shared only with the trusted parent process.
- GITCENTER_PIJUL_BIN: optional path to the Pijul executable; defaults to pijul.

Example:

    export GITCENTER_PIJUL_ROOT=/srv/gitcenter/pijul
    export GITCENTER_PIJUL_TOKEN="$(openssl rand -hex 32)"
    export GITCENTER_PIJUL_BIN=pijul
    ./target/release/gitcenter-pijul

The process reads one JSON request per line from stdin and writes one JSON response per line to stdout. Diagnostics go to stderr. Keep stdin/stdout private to the trusted parent process. Do not expose this process through a public TCP listener.

## Supported operations

- version: reports the bridge protocol and pinned Pijul compatibility target.
- init: initializes an already-existing, empty directory below the configured root, on channel main.
- inspect: validates a real Pijul repository and returns channels plus the latest change identifier, if present.

Repository paths are relative to GITCENTER_PIJUL_ROOT. Absolute paths, parent traversal, and paths resolving outside the managed root are rejected. Init does not create parent directories and does not initialize a non-empty directory.

See ../../docs/PIJUL-02-BRIDGE-PROTOCOL.md for the complete wire contract and security boundary.

## Current limitations

- Read-only metadata inspection plus initialization only.
- No channel mutation, file browsing, file reads, change application, transport, signing, or remote credentials.
- CLI text parsing is isolated in this adapter and covered by the pinned compatibility target. It must be replaced or strengthened with structured output/native APIs before the bridge grows beyond this narrow foundation.
- Runtime integration against an installed Pijul binary has not been executed in this repository-editing environment.
