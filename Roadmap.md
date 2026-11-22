# GitCenter Roadmap

This roadmap records planned product capabilities. Items are proposals, not implemented features, unless explicitly marked otherwise.

## Repository experience

- [ ] **Contributors widget**
  - Contributor avatars, names/identifiers, and contribution counts.
  - A contribution-activity graph over time.
  - Separate attribution for commit authors, committers, and co-authors.
  - Contributor details and time-range filtering.
  - Git history as the primary source of contribution data.

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
- Preserve the distinction between commit metadata and cryptographically authenticated identity.
- Make attribution explainable, privacy-conscious, and resistant to misleading claims.
- Documentation does not imply implementation.
