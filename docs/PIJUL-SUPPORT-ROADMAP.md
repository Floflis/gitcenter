# Pijul Support Roadmap

## Goal

Add native Pijul repository support to GitCenter while preserving existing Git and Mercurial repositories and their behavior.

GitCenter should become VCS-agnostic at the application/UI boundary and remain native to each supported VCS at the storage and operation layers. Pijul must not be represented internally as Git commits, trees, blobs, branches, or parent commits merely to fit the existing Git-oriented code.

This document is a staged implementation plan, not a claim that Pijul support already exists.

## Current architecture and constraints

The current repository implementation is centered around `main/dist/js/repo.js`:

- Repository initialization selects Git or Mercurial based on repository metadata (`content.git` / `content.hg`).
- The selected implementation is exposed through `repo.vcs`.
- File browsing assumes a commit resolves to a tree and files resolve to blobs.
- File editing creates a new VCS commit and advances a branch reference.
- Shared UI code exposes branch/tag selection and permanent links using the existing VCS abstractions.

The current adapters are `main/dist/js/git.js` and `main/dist/js/hg.js`.

Pijul has a different model:

- Changes are first-class, identified objects.
- A channel represents a named view/state assembled from changes; it is not a Git branch.
- Change dependencies and application order are part of the native model.
- Conflicts and concurrent changes must be represented using Pijul semantics.
- Author signatures and remote transport authentication are separate concerns.

Consequently, a Pijul adapter must not fabricate Git-style commit ancestry or force Pijul data into Git object terminology.

## Target architecture

Introduce a VCS provider boundary between the GitCenter UI/repository layer and each VCS implementation.

Conceptual layout:

```text
GitCenter UI
  |
Repository / VCS provider interface
  |-- Git provider
  |-- Mercurial provider
  `-- Pijul provider
        |
        `-- Rust bridge / Pijul-native storage and operations
```

The shared interface should expose semantic operations needed by the UI, while allowing each provider to return VCS-specific metadata.

Suggested initial interface (names are illustrative and should be aligned with the existing code during PIJUL-01):

- `getInfo()`
- `listViews()`
- `listFiles(view, path)`
- `readFile(view, path)`
- `listChanges(view, options)`
- `getChange(changeId)`
- `getChangeDiff(changeId)`
- `getPermanentReference(view, path)`

Use a neutral concept such as `view` at the UI boundary. Git can map it to branches/tags, Mercurial to its supported named views, and Pijul to channels. Do not imply these concepts are semantically interchangeable.

Keep provider-specific capabilities explicit. The interface must not promise operations that a backend cannot safely implement.

## Pijul backend

Add a separate Rust component, provisionally named `gitcenter-pijul`, to perform Pijul-native repository operations.

Responsibilities:

- Initialize and inspect repositories.
- List and manage channels, subject to authorization and safety checks.
- Read channel-specific file trees and file contents.
- List and inspect changes, including metadata and dependencies.
- Produce structured diffs.
- Apply changes and report conflicts without hiding or fabricating them.
- Coordinate remote pull/push through supported Pijul transports.
- Enforce repository path boundaries, input validation, and concurrency controls.

The bridge should use supported Pijul Rust APIs where practical. If CLI commands are used for an initial implementation, pin and document the supported Pijul version, invoke commands without a shell, validate all arguments, and parse stable machine-readable output where available. Do not depend on undocumented human-readable output.

Do not let a shared mutable working copy be switched between channels for concurrent web requests. Channel-specific reads must be isolated, snapshot-based, or otherwise safe under concurrent access. Any initial limitation must be documented and tested.

The bridge API should be narrow, versioned, and authenticated. The browser must not receive arbitrary command-execution capability.

## UI and product behavior

### Code browsing

Retain the familiar GitCenter code-browsing experience:

- Directory navigation and breadcrumbs.
- File viewing and syntax highlighting.
- README rendering.
- Download support where safe.
- A channel selector for Pijul repositories.

The selected channel determines the file view. Do not label channels as branches in Pijul-specific UI.

### Changes

Add a Pijul-native changes history:

- Change identifier.
- Author identity and signature-verification status.
- Message and description.
- Timestamp, when available.
- Dependencies.
- Affected paths.
- Structured diff.
- Channels containing or referencing the change, when this information is available.

A timeline is a suitable initial presentation. A dependency visualization can be added later. Do not manufacture a Git commit graph.

### Discussions and contributions

Use a discussion-based contribution workflow inspired by Pijul Nest:

- A contributor opens a discussion describing the proposed work.
- The contributor sends or attaches Pijul changes to the target repository/channel.
- The discussion displays the submitted changes and their diffs.
- Reviewers comment and discuss the changes.
- Authorized maintainers decide whether and when to apply them.
- The discussion records application status and relevant change identifiers.

This is not a GitHub Pull Request implemented on top of fake branches. Keep Git/Hg Pull Request behavior unchanged where it already exists; provide Pijul-appropriate contribution terminology and workflow.

The exact protocol for associating incoming changes with a discussion must be specified and tested before implementation. Do not assume a particular Pijul Nest command or wire protocol is compatible with the selected Pijul version without verification.

### Issues

Preserve the existing issue system where it is independent of Git-specific object semantics. Audit its dependencies before enabling it for Pijul repositories.

### Releases

Represent a release using GitCenter metadata that points to a reproducible Pijul state, such as a channel plus an explicit set of change identifiers or another stable native reference supported by the chosen Pijul version.

Do not pretend that a Git tag or commit SHA exists for a Pijul release.

## Identity, signatures, and transport

Keep these concepts distinct:

1. GitCenter account identity.
2. Pijul change-author/signing identity.
3. Remote transport credentials (for example, SSH credentials).

A GitCenter account may link to one or more Pijul identities, but the association must be verifiable and must not overwrite the original author identity or signature.

The server may store, serve, replicate, and verify changes. It must not silently re-sign contributors' changes as the server or claim server authorship on their behalf.

Transport capabilities must be verified against the exact Pijul version selected for implementation. In particular, do not assume that HTTP receive/fetch support implies HTTP push support. If push requires SSH or a dedicated server process, document and implement that transport explicitly.

The browser must not receive private signing keys or unrestricted SSH credentials. Define a secure credential and authorization boundary for remote operations.

## ZeroNet integration

Preserve GitCenter's existing ZeroNet integration, but do not assume that placing a raw `.pijul` directory inside a published site automatically provides correct decentralized replication.

Design publication and replication separately:

- Define which repository data is published.
- Define how changes and metadata are discovered and verified.
- Preserve original change signatures.
- Define how channels and discussions are represented in published metadata.
- Handle partial availability and stale replicas.
- Prevent untrusted published metadata from becoming executable instructions.
- Ensure publication does not require one central operator to sign every contributor's work.

The exact replication mechanism is an architectural task and must be validated with a working prototype before it is described as implemented.

## Pull request roadmap

### PIJUL-01 — VCS provider boundary

- Audit all VCS-specific assumptions in `repo.js` and shared UI code.
- Introduce a semantic provider interface.
- Adapt Git and Mercurial without changing their existing behavior.
- Replace Git-specific UI assumptions only where required by the shared interface.
- Add regression tests or documented manual test cases for existing Git/Hg repositories.

**Acceptance:** Git and Hg repositories continue to browse and perform their currently supported operations. No Pijul implementation is faked in this PR.

### PIJUL-02 — Rust bridge and storage foundation

- Add the `gitcenter-pijul` component.
- Pin the Pijul version/API compatibility target.
- Define a versioned, authenticated bridge protocol.
- Implement repository discovery, initialization, and basic metadata inspection.
- Add path validation, error mapping, logging, and integration tests.

**Acceptance:** A test harness can initialize and inspect a real Pijul repository through the bridge. No browser-facing feature is claimed yet.

### PIJUL-03 — Repository registration and lifecycle

PIJUL-03A defines the metadata/type contract. PIJUL-03B must first validate how a Pijul repository can be initialized in GitCenter's existing ZeroNet storage model. Do not assume a local supervisor or a separate bridge-managed root is necessary merely to create the repository directory.

#### PIJUL-03A — Metadata contract and identification

- Define the signed `content.json` metadata object for Pijul repositories.
- Validate metadata versions, storage identifier syntax, and native VCS type exclusivity.
- Identify Pijul repositories without constructing a fake Git/Hg backend.
- Preserve existing Git/Hg metadata and migration behavior.
- Treat the current `storage_path` semantics as provisional until PIJUL-03B establishes whether the native repository belongs inside the ZeroNet site tree or in separately managed storage.

**Acceptance:** GitCenter can identify a correctly declared Pijul repository and reject invalid/mixed backend metadata without changing existing repository types. This step does not claim bridge-backed registration.

#### PIJUL-03B — Native initialization and storage decision

- Audit the existing `ZeroFS` API and GitCenter's direct Git initialization implementation.
- Generate two empty repositories using the exact pinned Pijul version `1.0.0-beta.24`.
- Capture and compare their complete `.pijul` trees, file contents, sizes, and hashes.
- Determine which files are constant, generated, binary, or lock/runtime state.
- Verify that a proposed empty pristine database fixture can be copied and reopened safely, or identify the minimal native initialization operation required to create it.
- Prototype writing the verified initial layout through `ZeroFS`, following the existing Git initialization model.
- Keep Pijul-native operations (pristine access, channels, changes, recording, and transport) separate from filesystem bootstrap.
- Revisit the metadata schema's storage location only after an in-site versus external-storage prototype is validated.

**Acceptance:** GitCenter can create a valid empty Pijul repository in the chosen storage domain, and the exact pinned Pijul binary can reopen and inspect it. No placeholder `pristine/db`, guessed metadata, or unverified historical layout is accepted.

A local supervisor is a possible requirement only if the verified pristine initialization or subsequent native operations require a process/library unavailable to the ZeroNet runtime. It is not a prerequisite assumed for creating `.pijul`.

Research and evidence: `docs/PIJUL-INIT-RESEARCH.md`.
 
### PIJUL-04 — Channels and code browsing

- List channels and select a channel.
- Implement channel-specific directory and file reads.
- Add safe concurrent read isolation.
- Add file-view and navigation UI.
- Add stable, native Pijul references for browsed content where supported.

**Acceptance:** Users can browse files from a selected channel without changing a shared working copy or mislabeling channels as branches.

### PIJUL-05 — Changes history and diffs

- List changes with pagination.
- Display change metadata and dependencies.
- Display structured diffs and affected paths.
- Show signature verification state.
- Add links to changes and stable references.

**Acceptance:** Users can inspect real Pijul changes and diffs; identifiers and metadata come from Pijul rather than synthetic Git commits.

### PIJUL-06 — Discussions and incoming contributions

- Define the discussion-to-change association protocol.
- Implement incoming change discovery/receipt.
- Display attached changes and diffs in discussions.
- Add review comments and contribution status.
- Define authorization for maintainers and contributors.

**Acceptance:** A contributor can submit native Pijul changes for review, and reviewers can inspect them without converting them to Git commits.

### PIJUL-07 — Applying changes and conflict reporting

- Implement authorized change application.
- Preserve dependency information and original signatures.
- Report conflicts explicitly.
- Add safe recovery and failure handling.
- Test concurrent submissions and stale channel state.

**Acceptance:** Maintainers can apply an accepted contribution through Pijul-native operations, with conflicts and failures visible and recoverable.

### PIJUL-08 — Remote transport and credentials

- Select and document supported remote transport(s).
- Implement authenticated push/pull operations.
- Separate account identity, signing identity, and transport credentials.
- Add permission checks, audit logging, and credential protection.
- Test interrupted transfers and retry behavior.

**Acceptance:** Supported remote operations work with documented transport requirements and without exposing private credentials to the browser.

### PIJUL-09 — ZeroNet publication and replication

- Define the published repository representation.
- Implement discovery and replication of changes and signed metadata.
- Handle partial/stale replicas and verification failures.
- Define publication permissions and update behavior.
- Test replication between independent nodes.

**Acceptance:** Independent nodes can discover and verify published repository state without a central server re-signing contributors' changes.

### PIJUL-10 — Releases, compatibility, and documentation

- Add Pijul-native release references.
- Complete Git/Hg regression coverage.
- Add Pijul end-to-end tests and operational documentation.
- Document limitations, supported versions, backup/restore, and upgrade procedures.

**Acceptance:** The feature is documented, tested, and does not regress existing Git/Hg workflows.

## Cross-cutting requirements

Every implementation PR must:

- Include tests appropriate to the change.
- Include useful logs and actionable error messages.
- Document incomplete work in a TODO or follow-up issue.
- Avoid claiming support for unimplemented transport, replication, or review behavior.
- Preserve native repository data and signatures.
- Avoid silent Git/Pijul conversion.
- Keep credentials and signing keys out of browser-visible code.
- Document compatibility and migration implications.

## Out of scope for this roadmap

- Automatic conversion of Git or Mercurial repositories into Pijul.
- A universal commit abstraction that erases native VCS semantics.
- Replacing the existing Git/Hg workflows.
- Claiming ZeroNet replication merely because repository files are present in a published site.
- Implementing a full Pijul Nest fork.

## Reference projects and documentation

- GitCenter: https://github.com/Floflis/gitcenter
- Pijul: https://pijul.org/
- Pijul documentation: https://pijul.org/manual/
- Pijul Nest: https://nest.pijul.com/
- Pijul Nest source: https://nest.pijul.com/pijul/pijul

Before each implementation PR, verify the relevant API, command syntax, transport capabilities, and license/version compatibility against the exact upstream revision being targeted.
