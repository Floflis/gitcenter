# ZeroID Contributor Registry

## Status

Planned. This document specifies a design proposal; it does not describe an existing registry or implementation.

## Goal

Allow a ZeroNet identity (ZeroID) to publish a cryptographically signed declaration associating that identity with an email address. GitCenter can compare the declared address with email metadata in Git commits and, on a match, display the identity's ZeroMe avatar.

The registry is an attribution aid, not an email-verification service.

## Core trust distinction

A valid signature can establish that the declaration was authorized by the holder of the relevant ZeroID signing key, assuming the ZeroID key and signature verification are valid.

It does **not** establish that:

- The signer owns or controls the declared email inbox.
- The email provider has verified the address.
- A commit containing that email was actually authored by the ZeroID holder.
- The Git commit metadata itself is authentic.

Git author and committer email fields are ordinary commit metadata and can be set by the commit creator. A registry match is therefore a signed, unilateral identity-to-email claim, not proof of email ownership or proof of commit authorship.

GitCenter must preserve this distinction everywhere the association is displayed.

## Declaration model

A declaration should contain, at minimum:

- Protocol/schema version.
- ZeroID identifier.
- Declared email address.
- Declaration creation time.
- A unique declaration identifier or nonce.
- Signature produced by the ZeroID identity key.

The signed payload must have a canonical serialization and an explicit domain/context so a signature cannot be casually reused as authorization for an unrelated protocol or purpose. Verification must cover the exact payload fields, not an ambiguous reconstruction.

A future implementation must define key discovery, canonicalization, signature algorithms, revocation, expiration, and registry transport. This document intentionally does not assume a particular ZeroNet or ZeroMe API endpoint.

## Matching Git commits

GitCenter may normalize an email address for matching using a documented, conservative rule. It should not apply provider-specific transformations such as removing dots or plus-address suffixes unless a separate, explicit policy is designed.

Compare the declared address against email fields in commit metadata, including author and committer fields. Keep the role visible:

- Author email match: commit author metadata matches the declaration.
- Committer email match: commit committer metadata matches the declaration.

A match does not prove that the ZeroID holder created the commit. It only means the commit contains an email address that the ZeroID identity previously declared.

If multiple active ZeroID declarations claim the same email, do not arbitrarily choose one. Show an ambiguity state or omit the avatar association until a documented conflict policy exists.

## ZeroMe avatar display

When a declaration matches commit email metadata, GitCenter may retrieve and display the avatar associated with that ZeroID through a supported ZeroMe identity/profile mechanism.

Present the avatar as the avatar of the declaring ZeroID, not as proof that the commit was authored by that person. If the profile or avatar cannot be resolved, use a neutral fallback.

Do not fetch arbitrary avatar URLs supplied by commit metadata. Avatar resolution must follow the trusted ZeroID/ZeroMe lookup mechanism defined by the eventual implementation.

## Required user-facing disclosure

Every contributor entry using this registry association must expose a clear disclosure through a tooltip, accessible description, or contributor details:

> Email not verified. This ZeroID signed a declaration claiming this email address. The signature verifies the identity's declaration, not ownership of the email address. Git commit email metadata can also be set by the commit creator.

Use a concise visible status such as **Unverified email claim** near the identity where practical. The full explanation must be available on hover and keyboard focus, not hover alone.

Do not use labels such as “verified contributor”, “verified email”, or “authenticated author” for this association.

## Privacy and safety considerations

- Treat email addresses as personal data. Define whether declarations and email values are publicly readable before implementation.
- Make publication an explicit action by the ZeroID holder.
- Document how a declaration can be updated, revoked, or superseded.
- Avoid exposing the full email address in public contributor UI unless there is a deliberate product decision and clear user consent.
- Consider storing or indexing a privacy-preserving lookup value, while documenting that hashes of common email addresses may be guessable and are not automatically anonymous.
- Do not let a registry declaration override a user's local privacy settings or repository visibility.
- Handle malformed signatures, unknown keys, expired declarations, revoked declarations, and conflicting claims as non-matches or explicit warning states.

## Suggested status model

- **No registry match:** show Git contributor metadata without a ZeroID avatar association.
- **Signed, unverified email claim:** signature is valid and the declared email matches commit metadata, but email ownership is not verified.
- **Invalid declaration:** signature or payload validation failed; do not use the claimed identity or avatar.
- **Revoked/expired declaration:** do not use the association as active.
- **Ambiguous claim:** multiple identities claim the same email; do not select one silently.

## Acceptance criteria

- A ZeroID can sign a declaration associating it with an email address.
- GitCenter can verify the declaration signature against the ZeroID identity key.
- GitCenter can match the declared address against Git author/committer email metadata using documented normalization.
- A successful match can resolve and display the corresponding ZeroMe avatar through a supported lookup mechanism.
- Every such association is explicitly marked as an unverified email claim.
- The UI explains that the signature proves the ZeroID declaration, not email ownership or commit authorship.
- Invalid, revoked, expired, and ambiguous declarations never produce a silently trusted contributor identity.
- The feature does not claim to implement email verification.
