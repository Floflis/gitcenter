# Contributors widget

## Status

Planned. This document specifies product behavior; it does not describe an existing implementation.

## Goal

Provide a repository-level contributors experience familiar to GitHub users: a compact contributor summary with avatars, contribution totals, and an activity graph. The widget should make it easy to understand who participated and when, without implying that raw repository metadata is stronger identity proof than it really is.

The widget is intended to work across **Git, Mercurial, and Pijul**, rather than being tied to Git's commit model.

## Supported version-control systems

The widget should consume native history from:

- **Git:** commits and their author/committer metadata.
- **Mercurial:** changesets and their user/date metadata.
- **Pijul:** patches and the author metadata available in the repository's native history.

Use VCS-specific adapters to translate native history into a shared contribution model. Do not pretend that Git commits, Mercurial changesets, and Pijul patches are identical objects. The UI should use a neutral term such as **contribution** for cross-VCS summaries and retain the native object type in details and links.

Each adapter must document which metadata fields are available and how it derives contributor identity, timestamps, and contribution units. If a VCS does not expose a role or field, omit that role rather than inventing it.

## Widget contents

### Contributor list

Each entry should show:

- Avatar, when one can be resolved.
- Display name or repository-native contributor identity.
- Contribution count for the selected period.
- Optional role labels where supported by the source VCS, such as author, committer, or co-author.
- Contributor classification: Human, AI, or unclassified, when known.
- A link to the contributor detail view, when available.

Sort the default list by contribution count for the selected period, with deterministic tie-breaking. Allow inspection of the complete contributor list rather than silently hiding everyone beyond a small preview.

### Activity graph

Show contributions over time in a calendar-style activity graph, grouped by day or week. Include a selectable time range and hover/focus details such as date and contribution count. Empty periods must remain visible.

The default contribution unit is one native history item attributed to a contributor: a Git commit, Mercurial changeset, or Pijul patch. State the native unit or normalized counting method explicitly. Do not mix commits, patches, changed lines, reviews, and other events into one unexplained score.

Where one native item has multiple contributors, define and disclose the counting rule. Avoid inflating repository-wide totals when showing multiple contributor associations.

### Humans | AI | All tabs

Add a compact pill-style tab selector with exactly these labels:

**Humans | AI | All**

- **Humans:** show contributions attributed to contributors classified as human.
- **AI:** show contributions attributed to contributors classified as AI agents or AI systems.
- **All:** show both groups together.

The selected tab must affect the contributor list, totals, and activity graph consistently. The All view should make the human/AI split inspectable, for example through per-group totals or a legend.

A sensible initial default is **Humans**, so repositories with substantial automated or AI-generated activity do not overwhelm the human contributor view. Users can switch to AI or All at any time.

#### Classification and trust

Do not guess whether an identity is human or AI based on its display name, email address, avatar, writing style, or commit message.

Classification should come from an explicit, documented source, such as a declared contributor profile or a future machine/agent identity registry. The source and status of the classification should be inspectable. A repository maintainer's manual classification may be supported later, but must be visibly identified as maintainer-provided rather than cryptographically verified.

Contributors without a reliable classification should remain **unclassified**. Include them in **All**, but do not silently place them in Humans or AI. The UI should make the presence of unclassified activity visible so that the filtered tabs do not create a misleading picture.

An AI system may contribute through a human-operated account or shared identity. Do not automatically classify every contribution from that account as AI; classification needs to apply to the actual contributor identity or explicitly attributed contribution, with provenance.

### Contributor details

A detail view may summarize:

- Total native contributions in the selected range.
- First and most recent contribution dates.
- Author, committer, and co-author counts separately, where supported.
- Recent commits, changesets, or patches attributed to that contributor.
- Human/AI classification and its provenance, when available.

Do not treat lines added/deleted as a measure of a person's or AI system's value or quality. If shown, present them as neutral repository statistics, separate from contribution ranking.

## Attribution rules

Repository history metadata is not inherently verified identity data. Git author/committer fields, Mercurial user metadata, and Pijul author metadata may be supplied or configured by the actor creating or recording the change.

Keep available roles distinct:

- **Author:** identity recorded as having authored the change.
- **Committer:** identity recorded as having created or applied the Git commit, where applicable.
- **Co-author:** an additional author explicitly recorded through supported metadata.

A person or agent may appear in more than one role. Avoid counting the same native history item multiple times in the headline total merely because the same contributor appears in multiple roles. Role-specific breakdowns can still show each relationship.

When identities cannot be confidently unified, retain separate contributor entries rather than guessing that two names or email addresses belong to the same person.

## Avatar resolution

Avatar sources may include a future ZeroID registry integration. An avatar match must not silently upgrade an unverified email declaration into a verified identity claim. See [ZeroID Contributor Registry](zeroid-contributor-registry.md).

If no supported avatar is available, use a neutral fallback. Do not derive an avatar from an arbitrary remote URL embedded in repository metadata.

## Interface and accessibility

- Make the widget useful on narrow screens and desktop.
- Render Humans, AI, and All as a compact pill-style tab selector with a clear selected state.
- Provide text labels and accessible descriptions for graph values; do not communicate activity through color alone.
- Ensure tabs, contributor entries, and graph cells are keyboard navigable.
- Make tooltips available on hover and keyboard focus.
- Clearly label the selected time range, contributor classification, and counting method.
- Preserve consistent filtering across contributor list, totals, and activity graph.

## Non-goals for the initial version

- Scoring contributor quality, importance, or project ownership.
- Inferring real-world identity from names or email addresses.
- Inferring human/AI status from names, avatars, email addresses, or writing style.
- Treating contribution counts as a complete measure of non-code work or value.
- Claiming repository metadata has been independently authenticated.
- Flattening the native history models of Git, Mercurial, and Pijul into falsely interchangeable concepts.

## Acceptance criteria

- A repository page can present a contributor list and time-based activity graph for Git, Mercurial, and Pijul repositories.
- VCS-specific adapters preserve native history semantics while exposing a consistent cross-VCS contribution view.
- The contribution unit and selected period are explicit.
- The Humans | AI | All pill tabs filter the list, totals, and graph consistently.
- Contributors without reliable classification remain unclassified and are included in All without being guessed into Humans or AI.
- Author, committer, and co-author attribution are distinguished where the source VCS supports them.
- Duplicate role appearances do not inflate the headline count for one native history item.
- Unresolved identities remain separate instead of being merged by guesswork.
- Avatar provenance and identity-verification limitations are communicated clearly.
