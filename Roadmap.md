# GitCenter Roadmap

This roadmap records planned product capabilities. Items are proposals, not implemented features, unless explicitly marked otherwise.

## Repository experience

- [ ] **Cross-VCS Contributors widget**
  - Support contributor summaries and activity visualization for Git, Mercurial, and Pijul repositories.
  - Display contributor avatars, names/identifiers, and contribution counts.
  - Provide a contribution-activity graph over time.
  - Separate attribution roles where the source VCS exposes them.
  - Add the **Humans | AI | All** filter tabs to distinguish human and AI contributions and inspect the combined activity.
  - Provide contributor details and time-range filtering.
  - Use native repository history through VCS-specific adapters, normalized into a shared contribution model.

## Identity and attribution

- [ ] **ZeroID Contributor Registry**
  - A ZeroNet identity (ZeroID) signs a declaration associating that identity with an email address.
  - GitCenter matches declared addresses against Git commit author and committer email metadata.
  - On a match, display the corresponding ZeroMe identity/avatar.
  - Clearly label the association **unverified**: the signature proves control of the signing identity/key, not ownership or control of the email inbox.
  - Explain this distinction in tooltips and contributor details.

## Documentation

- [Contributors widget](docs/contributors-widget.md)
- [ZeroID Contributor Registry](docs/zeroid-contributor-registry.md)

## Principles

- Never present an identity-to-email declaration as email verification.
- Preserve the distinction between repository metadata and cryptographically authenticated identity.
- Do not infer whether a contributor is human or AI from names, avatars, or email strings alone.
- Make attribution explainable, privacy-conscious, and resistant to misleading claims.
- Documentation does not imply implementation.
