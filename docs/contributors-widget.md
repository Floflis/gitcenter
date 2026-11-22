# Contributors widget

## Status

Planned. This document specifies product behavior; it does not describe an existing implementation.

## Goal

Provide a repository-level contributors experience familiar to GitHub users: a compact contributor summary with avatars, contribution totals, and an activity graph. The widget should make it easy to understand who participated and when, without implying that raw Git metadata is stronger identity proof than it really is.

## Widget contents

### Contributor list

Each entry should show:

- Avatar, when one can be resolved.
- Display name or Git author identity.
- Contribution count for the selected period.
- Optional role labels: author, committer, and co-author.
- A link to the contributor detail view, when available.

Sort the default list by contribution count for the selected period, with deterministic tie-breaking. Allow inspection of the complete contributor list rather than silently hiding everyone beyond a small preview.

### Activity graph

Show contributions over time in a calendar-style activity graph, grouped by day or week. Include a selectable time range and hover/focus details such as date and contribution count. Empty periods must remain visible.

The default contribution unit is commits attributed to a contributor. State the counting unit explicitly. Do not mix commits, changed lines, reviews, and other events into one unexplained score.

### Contributor details

A detail view may summarize:

- Total commits in the selected range.
- First and most recent contribution dates.
- Author, committer, and co-author counts separately.
- Recent commits attributed to that contributor.

Do not treat lines added/deleted as a measure of a person's value or quality. If shown, present them as neutral repository statistics, separate from contribution ranking.

## Attribution rules

Git commit metadata is not inherently verified identity data. Author and committer names and email addresses can be chosen by whoever creates a commit.

Keep these roles distinct:

- **Author:** identity recorded as having written the change.
- **Committer:** identity recorded as having created or applied the commit object.
- **Co-author:** an additional author explicitly recorded through supported co-author metadata.

A person may appear in more than one role. Avoid counting the same commit multiple times in the headline total merely because the person appears as both author and committer. Role-specific breakdowns can still show each relationship.

When identities cannot be confidently unified, retain separate contributor entries rather than guessing that two names or email addresses belong to the same person.

## Avatar resolution

Avatar sources may include a future ZeroID registry integration. An avatar match must not silently upgrade an unverified email declaration into a verified identity claim. See [ZeroID Contributor Registry](zeroid-contributor-registry.md).

If no supported avatar is available, use a neutral fallback. Do not derive an avatar from an arbitrary remote URL embedded in commit metadata.

## Interface and accessibility

- Make the widget useful on narrow screens and desktop.
- Provide text labels and accessible descriptions for graph values; do not communicate activity through color alone.
- Ensure contributor entries and graph cells are keyboard navigable.
- Make tooltips available on hover and keyboard focus.
- Clearly label the selected time range and counting method.

## Non-goals for the initial version

- Scoring contributor quality, importance, or project ownership.
- Inferring real-world identity from names or email addresses.
- Treating commit counts as a complete measure of non-code work.
- Claiming Git metadata has been independently authenticated.

## Acceptance criteria

- A repository page can present a contributor list and time-based activity graph.
- The contribution unit and selected period are explicit.
- Author, committer, and co-author attribution are distinguishable.
- Duplicate role appearances do not inflate the headline count for a single commit.
- Unresolved identities remain separate instead of being merged by guesswork.
- Avatar provenance and identity-verification limitations are communicated clearly.
