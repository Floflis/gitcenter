# PIJUL-03A: repository metadata and identification

## Scope

This stage defines how GitCenter identifies a Pijul repository in signed site metadata and how the existing application distinguishes it from Git and Mercurial. It does not claim that the trusted bridge is connected to the ZeroNet browser runtime.

The current GitCenter Git implementation creates its repository layout directly through ZeroFS; it does not invoke `git init`. The PIJUL-02 bridge currently uses a separately configured filesystem root, but that implementation choice does not prove that Pijul repositories must live outside the ZeroNet site tree. The storage location remains provisional pending the exact-version ZeroFS initialization experiment documented in `docs/PIJUL-INIT-RESEARCH.md`.

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
- `storage_path` is currently a portable relative identifier in the provisional bridge-managed storage design. Its final meaning (ZeroNet site-relative versus bridge-root-relative) is intentionally unresolved until PIJUL-03B validates a real initialization prototype.
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

The final sequence depends on PIJUL-03B's storage experiment. The first candidate is to initialize the verified empty `.pijul` layout through the same ZeroFS storage API that GitCenter already uses for its native Git implementation.

If the pinned Pijul release requires a native runtime operation to create the pristine database, only that operation should be delegated to a trusted native component. Do not introduce a local supervisor or move all repository data outside the site tree unless the prototype demonstrates that this is necessary.

The site owner must sign and publish `content.json` only after the repository has been validated by the exact pinned Pijul implementation. The browser must never choose an arbitrary host path or claim registration success based only on user-supplied JSON.

## Current implementation status

- Pijul metadata schema is defined.
- The repository loader recognizes valid Pijul metadata and rejects mixed Git/Hg/Pijul declarations.
- `getVcsInfo()` reports Pijul as declared but unavailable while bridge transport is not connected.
- The generic `install(..., "pijul")` path fails explicitly instead of silently succeeding or writing unverified metadata.
- Git/Hg installation continues to use its existing native implementations.

## Required follow-up: PIJUL-03B

PIJUL-03B must first produce and validate an empty `1.0.0-beta.24` fixture and prove whether it can be installed through ZeroFS. The modern upstream format documents a Sanakirja pristine database, so an empty directory tree or placeholder `pristine/db` is not a valid repository.

Only after that experiment should the project decide whether repository creation needs a native service, whether the repository can live in the ZeroNet site tree, and what `storage_path` means. See `docs/PIJUL-INIT-RESEARCH.md`.

## Non-goals

This stage does not implement Pijul file browsing, channel selection, change history, diffs, transport, signing, ZeroNet replication, or a public bridge endpoint.
