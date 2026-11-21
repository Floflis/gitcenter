# PIJUL-01 implementation notes: VCS provider boundary

## Scope

This change introduces the first application-facing VCS provider seam without adding or pretending to add Pijul support. Existing Git and Mercurial backends remain responsible for their native storage and object semantics.

## Provider contract in this PR

The transitional `LegacyVcsProvider` in `main/dist/js/repo.js` exposes:

- `getInfo()`: identifies the native backend type.
- `listViews()`: returns the existing native refs/views used by the current UI.
- `listFiles(view, path)`: browses a directory from the selected native view.
- `readFile(view, path)`: reads a file from the selected native view.

The existing `Repository.getFiles()`, `getFile()`, and `getBranches()` methods remain as compatibility entry points and delegate to the provider. This preserves their current call signatures and result shapes while creating a provider seam for the shared browsing layer.

The neutral word `view` is used inside the provider contract. For this transitional adapter, Git refs and Mercurial's existing ref-like values retain their current native names and behavior. The UI must not infer that these concepts are semantically identical.

## Deliberate limitations

- This PR does not add a Pijul backend, repository registration, channels, changes, diffs, or transport.
- Write operations, commit creation, diff generation, hooks, and lower-level object access still use the existing Git/Hg paths. They are not yet represented as a complete cross-VCS contract.
- The provider is intentionally read-side first. The next refactor can move write/diff operations behind explicit capability-aware methods after their current call sites and behavior have been audited.
- No Git/Hg storage format, history, ref, or metadata is converted.

## Regression checks

Run these checks against existing repositories after loading GitCenter:

1. Open a Git repository and browse its root and a nested directory.
2. Open a text file and a binary/downloadable file.
3. Switch between an existing Git branch and tag and confirm the displayed tree matches the selected ref.
4. Repeat root, nested-directory, file, branch, and tag browsing against a Mercurial repository.
5. Verify Git file editing/commit, Mercurial file editing/commit, diff views, and Git hooks retain their pre-change behavior.
6. Open a repository without `git` or `hg` metadata and confirm browsing reports that no VCS provider is initialized rather than dereferencing a missing backend.

These are manual regression checks; this PR does not claim automated browser coverage.

## Follow-up

Before PIJUL-02, audit remaining direct `Repository.vcs`, `git`, and `hg` usage and define explicit capabilities for mutations, diffs, hooks, and submodules. Pijul-specific channel and change operations must be added as native capabilities, not mapped to synthetic Git commits or branches.
