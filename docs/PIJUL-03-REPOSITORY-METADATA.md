# PIJUL-03A: repository metadata and identification

## Scope

This stage defines how GitCenter identifies a Pijul repository in signed site metadata and how the existing application distinguishes it from Git and Mercurial. It does not claim that the trusted bridge is connected to the ZeroNet browser runtime.

The current GitCenter creation flow initializes Git/Hg data inside the merged ZeroNet site tree. PIJUL-02 instead defines a Rust process that owns a separately configured filesystem root and communicates over private stdin/stdout pipes. Those storage domains are not interchangeable.

## Metadata contract

A registered Pijul repository declares a top-level `pijul` object in its signed `content.json`:

    {
      "title": "Example",
      "description": "Pijul-backed repository",
      "installed": true,
      "pijul": {
        "schema_version": 1,
        "protocol_version": 1,
        "storage_path": "repositories/example",
        "pijul_version": "1.0.0-beta.24",
        "state": "registered"
      }
    }

The normative schema is `docs/schemas/pijul-repository-metadata.schema.json`.

- `schema_version` versions this metadata shape independently of the bridge.
- `protocol_version` identifies the bridge protocol required by this repository.
- `storage_path` is a portable relative identifier below the bridge's configured managed root. It is not an absolute host path and is not relative to the ZeroNet site directory.
- `pijul_version` records the exact Pijul compatibility target returned by bridge inspection.
- `state` is `registered` only after a trusted bridge inspection has confirmed the repository.

Do not add private keys, tokens, local absolute paths, SSH credentials, or mutable working-copy paths to signed public metadata.

## Native type exclusivity

A repository must declare exactly one native backend:

- Git: existing `git` metadata.
- Mercurial: existing `hg` metadata.
- Pijul: the new `pijul` metadata object.

A site declaring more than one backend is invalid and must not silently select one. Existing Git/Hg sites without Pijul metadata retain their current behavior.

The current repository loader validates the Pijul metadata shape and records the native type. It deliberately does not create a fake Git/Hg backend or expose file browsing when the Pijul bridge transport is unavailable.

## Registration lifecycle

The intended trusted registration sequence is:

1. The local GitCenter service allocates a storage identifier below its configured Pijul root.
2. The service asks the bridge to initialize or inspect that repository.
3. The bridge confirms the repository marker and exact Pijul version.
4. The service receives structured inspection metadata.
5. The GitCenter site owner signs and publishes `content.json` containing the Pijul metadata object.
6. On load, GitCenter validates the signed metadata and identifies the repository as Pijul.

The browser must never choose an arbitrary host path or claim registration success based only on user-supplied JSON.

## Current implementation status

- Pijul metadata schema is defined.
- The repository loader recognizes valid Pijul metadata and rejects mixed Git/Hg/Pijul declarations.
- `getVcsInfo()` reports Pijul as declared but unavailable while bridge transport is not connected.
- The generic `install(..., "pijul")` path fails explicitly instead of silently succeeding or writing unverified metadata.
- Git/Hg installation continues to use its existing native implementations.

## Required follow-up: PIJUL-03B

The remaining registration UI and lifecycle operations require a trusted local service/supervisor that launches `gitcenter-pijul`, owns the shared secret, forwards fixed protocol requests over private pipes, and returns authenticated/validated results to the GitCenter application. The browser must not receive the secret or spawn the process.

PIJUL-03B must implement and test that integration before adding a Pijul option to the creation form or writing `state: registered` metadata. It must also define service availability, timeout, cancellation, duplicate registration, cleanup, and recovery behavior.

## Non-goals

This stage does not implement Pijul file browsing, channel selection, change history, diffs, transport, signing, ZeroNet replication, or a public bridge endpoint.
