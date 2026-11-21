# PIJUL-02: gitcenter-pijul bridge protocol

## Compatibility contract

| Item | Contract |
| --- | --- |
| Bridge package | gitcenter-pijul 0.1.0 |
| Wire protocol | JSON Lines, protocol version 1 |
| Pijul CLI | 1.0.0-beta.24 |
| Transport | Private stdin/stdout pipes between trusted local processes |
| Authentication | Shared high-entropy bearer secret on every request |
| Repository addressing | Relative path below a configured canonical root |

Pijul 1.0.0-beta.24 is the selected compatibility target for this implementation. The bridge checks the executable version at inspection time and rejects a mismatch. Updating Pijul requires an explicit compatibility change and tests.

The bridge uses the documented Pijul CLI for this first slice rather than claiming an unverified direct libpijul API integration. The subprocess is always invoked with argument arrays through std::process::Command; shell interpretation is never involved. The command surface is fixed by the bridge and cannot be selected by the caller.

## Framing

Each request is one UTF-8 JSON object terminated by a newline. Each response is one JSON object terminated by a newline. A malformed line produces an invalid_request response and does not terminate the process.

Request:

    {
      "protocol_version": 1,
      "request_id": "req-0001",
      "token": "<shared-secret>",
      "operation": "inspect",
      "repository": "projects/example"
    }

Success:

    {
      "protocol_version": 1,
      "request_id": "req-0001",
      "ok": true,
      "result": {
        "repository": "projects/example",
        "pijul_version": "1.0.0-beta.24",
        "channels": ["main", "review"],
        "current_channel": "main",
        "latest_change": "<native-change-hash-or-null>"
      }
    }

Failure:

    {
      "protocol_version": 1,
      "request_id": "req-0001",
      "ok": false,
      "error": {
        "code": "not_a_pijul_repository",
        "message": "repository has no readable .pijul directory"
      }
    }

The request_id is opaque to the bridge and is echoed unchanged. The response protocol version is always the bridge's supported version, including for unsupported request versions.

## Operations

### version

Returns bridge_version, protocol_version, and the pinned pijul_version. Authentication is still required.

### init

Requires repository to identify an already-existing, empty directory below the configured root. Initializes it with the main channel, then performs the same real-repository inspection as inspect. It never initializes the managed root itself, creates parent directories, or overwrites an existing repository.

### inspect

Requires an existing directory containing a real .pijul directory. Returns channel names, the active channel when the CLI marks one, and the latest change hash when available. It does not switch channels or modify repository state.

## Authentication and process boundary

- GITCENTER_PIJUL_TOKEN is supplied to the process by its trusted parent and must contain at least 32 bytes.
- Every request carries the same shared secret in token; comparison is implemented without content-dependent early exit.
- The secret must never be sent to the browser, written to logs, or committed to repository files.
- The bridge is intended to be a child process over private pipes, not a network daemon. A future network transport must add authenticated encryption, replay controls, request limits, and explicit peer authorization before deployment.
- Stdout is protocol-only. Diagnostics are written to stderr.

## Filesystem boundary

- GITCENTER_PIJUL_ROOT is canonicalized at startup.
- Callers supply only relative repository paths.
- Absolute paths, parent traversal, and non-normal path components are rejected.
- Existing repository paths and their .pijul markers are canonicalized and checked against the managed root.
- Init requires a pre-created, empty directory inside the managed root.
- This protects against ordinary path traversal and symlink escapes at validation time. It is not a defense against a hostile local actor racing filesystem changes between validation and command execution; the parent service must own and protect the managed root.

## Error codes

invalid_request, unsupported_protocol, unauthorized, missing_repository, invalid_path, path_outside_root, repository_not_found, repository_not_empty, not_a_pijul_repository, unsupported_pijul_version, pijul_unavailable, pijul_command_failed, invalid_pijul_output, and unsupported_operation.

Messages are diagnostic text, not a stable machine interface. Clients must branch on code.

## Explicit non-goals for PIJUL-02

No browser integration, GitCenter repository registration, file-tree browsing, file reads, channel switching, changes/diffs, push/pull, SSH credential management, signing, or ZeroNet publication is implemented here. Those belong to later roadmap steps and must use native Pijul semantics.
