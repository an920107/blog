---
name: git-conventions
description: "Enforce this repository's Git commit message and branch naming conventions. Use when creating a commit, writing a commit message, or creating/naming a branch. Commit messages must be `BLOG-xxx action: summary` or `NO-ISSUE action: summary`; branches must be `BLOG-xxx_the_purpose` or `NO-ISSUE_the_purpose`."
---

# Git Commit & Branch Conventions

This repository requires a ticket prefix on every commit message and branch name.
Apply these rules whenever you create a commit or a branch.

## Commit messages

Format:

```
BLOG-xxx action: did something in this commit
NO-ISSUE action: did something in this commit
```

- Start with a ticket reference: `BLOG-<number>` when the work maps to a real
  ticket, or `NO-ISSUE` for work with no ticket (chores, version bumps, syncing
  branches, small fixes).
- Follow with a single space, then a short lowercase `action` word describing
  the change (for example `add`, `fix`, `update`, `remove`, `refactor`,
  `migrate`, `chore`).
- Then a colon (`:`) and a concise one-line summary of what the commit does.
- Do not end the subject line with a period.

Examples:

```
BLOG-205 add: post social share button
BLOG-305 fix: og metadata fetch on share
BLOG-290 migrate: frontend JS runtime to bun
NO-ISSUE chore: bump version to 0.7.3
NO-ISSUE update: sync main with release/0.7
```

## Branch names

Format:

```
BLOG-xxx_the_purpose_of_the_branch
NO-ISSUE_the_purpose_of_the_branch
```

- Use the same ticket prefix as the commit messages.
- Separate the ticket from the purpose with a single underscore (`_`).
- Write the purpose in lowercase, with words separated by underscores (`_`).
- Keep the purpose short and descriptive of the branch's goal.

Examples:

```
BLOG-205_social_share
BLOG-296_add_mcp_servers_for_antigravity
BLOG-295_fix_og_metadata_fetch_issue
NO-ISSUE_update_readme
```

## How to apply

1. Determine the ticket number. Use `BLOG-<number>` when one is known. If there
   is genuinely no ticket, use `NO-ISSUE`.
2. If the ticket reference is unclear, ask the user before committing or
   branching rather than guessing a number.
3. Build the commit subject as `<ticket> <action>: <summary>` and the branch
   name as `<ticket>_<purpose>`.
4. When there are multiple logical changes, prefer separate commits, each with
   its own correctly prefixed message.
5. When opening a pull request, keep the ticket prefix at the start of the PR
   title so it matches the commit convention.

Do not use imperative-mood-without-prefix messages (for example `Fix bug`) and
do not omit the ticket prefix. Every commit and branch must start with
`BLOG-xxx` or `NO-ISSUE`.
