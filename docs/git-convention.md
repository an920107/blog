# Git Convention

This document describes how commits, branches, and pull requests are named in this
repository, and which parts of the convention are enforced automatically.

The short version: **every commit message and every branch name starts with a ticket
reference — `BLOG-<number>`, or `NO-ISSUE` when there is genuinely no ticket.**

Anything starting with `BLOG-` or `NO-ISSUE` is easy to grep, easy to link back to
the project board, and easy to review later when you are reading `git log` and
wondering why a change exists. That is the whole point of the prefix; the rest of
the format is about keeping the log scannable.

> This document is the human-readable source of truth. The same rules are mirrored
> for AI agents in `.agents/skills/git-conventions/SKILL.md`; if you change the
> rules, update both.

## 1. Ticket references

| Prefix          | When to use it                                                                                              |
| --------------- | ----------------------------------------------------------------------------------------------------------- |
| `BLOG-<number>` | The work maps to a real ticket on the project board. This is the normal case.                               |
| `NO-ISSUE`      | Chores, version bumps, syncing branches, typo fixes, small cleanups — work that legitimately has no ticket. |

Do not invent a ticket number to satisfy the convention. If the ticket is unclear,
ask before committing or branching, and use `NO-ISSUE` only when there really is no
ticket behind the change.

## 2. Commit messages

### Format

```
<ticket> <action>: <summary>
```

Broken down:

```
BLOG-290 migrate: frontend JS runtime to bun
└──┬───┘ └──┬──┘ └──────────┬────────────┘
   │        │               └─ one-line summary, no trailing period
   │        └─ short lowercase action word
   └─ ticket reference, or NO-ISSUE
```

- Separate the ticket, the action, and the summary with single spaces.
- The action word must be **lowercase** and followed by a colon (`:`).
- Keep everything on one line. No trailing period.

### Action words

Prefer one of these; they keep the log predictable:

| Action     | Use it for                                           |
| ---------- | ---------------------------------------------------- |
| `add`      | A new feature, endpoint, page, or file               |
| `fix`      | A bug fix                                            |
| `update`   | A change to existing behaviour, content, or config   |
| `remove`   | Deleting code, dependencies, or files                |
| `refactor` | Restructuring without changing behaviour             |
| `migrate`  | Moving to a different runtime, framework, or storage |
| `chore`    | Tooling, dependencies, version bumps, housekeeping   |

If none of them fit, pick the closest word rather than writing a sentence.

### Examples

```
BLOG-205 add: post social share button
BLOG-305 fix: og metadata fetch on share
BLOG-290 migrate: frontend JS runtime to bun
BLOG-283 add: bm25 hybrid search
BLOG-323 add: redis db env
NO-ISSUE chore: bump version to 0.7.3
NO-ISSUE update: sync main with release/0.7
```

### Bad examples

```
Fix bug                                   # no ticket, no action word
BLOG-205: post social share button        # no action word
BLOG-205 Add: post social share button    # action word must be lowercase
BLOG-205 add: post social share button.   # no trailing period
BLOG-205 added share button               # no colon
```

### One commit, one idea

A commit should be reviewable on its own. If a change does two unrelated things, split
it into two commits, each with its own correctly prefixed message. This matters more
than commit count: prefer clear commits over a single "everything" commit.

## 3. Branch names

### Format

```
<ticket>_<purpose>
```

- The ticket part is identical to the one you use in commits.
- The separator is a single underscore (`_`), not a hyphen, slash, or space.
- The purpose is lowercase, words separated by underscores, and short enough to read
  at a glance.

### Examples

```
BLOG-205_social_share
BLOG-296_add_mcp_servers_for_antigravity
BLOG-295_fix_og_metadata_fetch_issue
BLOG-49_git_convention_document
NO-ISSUE_update_readme
NO-ISSUE_bump_version_0_8_0
```

### Long-lived branches

Work branches use the format above. Two kinds of branches are exempt because they are
structural, not work items:

- `main` — the integration branch.
- `release/<major>.<minor>` — release branches, for example `release/0.8`. Fixes that
  must ship without waiting for `main` are committed here and then synced back with
  `NO-ISSUE update: sync main with release/<version>`.

## 4. Pull requests

### Title

The PR title must also start with the ticket reference. This is checked by
[`.gitea/workflows/pr-title-check.yaml`](../.gitea/workflows/pr-title-check.yaml),
which fails the PR if the title does not match:

```
^(BLOG-[0-9]+|NO-ISSUE) [A-Z].+$
```

In practice that means: ticket reference, one space, then text starting with a capital
letter. Note this differs slightly from a commit message, where the action word is
lowercase.

```
BLOG-205 Add social share button        # passes
NO-ISSUE Update readme                  # passes
BLOG-205 add social share button        # fails: lowercase after the prefix
Add social share button                 # fails: no ticket reference
```

### Body

Fill in the template in [`.gitea/PULL_REQUEST_TEMPLATE.yaml`](../.gitea/PULL_REQUEST_TEMPLATE.yaml):

| Section         | Required | Notes                                                                             |
| --------------- | -------- | --------------------------------------------------------------------------------- |
| Description     | Yes      | What changed and why                                                              |
| Package Changes | Yes      | Dependency changes only, as a table of `Added` / `Removed` / `Updated`, or `None` |
| Screenshots     | No       | Required in spirit for anything user-visible                                      |
| Reference       | Yes      | `Resolves #<issue>`                                                               |
| Checklist       | Yes      | A milestone is set; the related issues are linked to the branch                   |

### Keeping `main` greppable

The goal is that `git log --oneline main` still shows ticket references. A squashed
merge does this naturally, because the resulting commit takes the PR title as its
subject — and the PR title is already required to start with the ticket reference. If
you merge with a merge commit instead, Gitea's default subject (`Merge pull request
'...'`) buries the prefix in the middle, so either edit the merge title or squash.

## 5. What is checked automatically

| Check                                               | Where                                  | Blocks the change       |
| --------------------------------------------------- | -------------------------------------- | ----------------------- |
| PR title format                                     | `.gitea/workflows/pr-title-check.yaml` | Yes                     |
| Backend `cargo check`, `cargo clippy`, `cargo test` | `.gitea/workflows/backend-ci.yaml`     | Yes                     |
| Frontend ESLint and `bun run check`                 | `.gitea/workflows/frontend-ci.yaml`    | Yes                     |
| `sqlx` offline query cache is up to date            | `script/pre-commit/sqlx-prepare.sh`    | Locally, via pre-commit |
| Backend compile check                               | `script/pre-commit/backend-check.sh`   | Locally, via pre-commit |
| Frontend lint and check                             | `script/pre-commit/frontend-lint.sh`   | Locally, via pre-commit |

Set the hooks up once per clone:

```bash
pip install pre-commit
pre-commit install
```

Commit message format itself is not enforced by CI — it is a review-time convention.
Reviewers should ask for a reword (`git commit --amend`, or an interactive rebase) when
a subject line does not follow the format.

## 6. Before you push

- [ ] Branch name is `<ticket>_<purpose>`.
- [ ] Every commit subject is `<ticket> <action>: <summary>`, lowercase action word, no trailing period.
- [ ] Unrelated changes are in separate commits.
- [ ] Pre-commit hooks pass (they run automatically once installed).
- [ ] PR title starts with `BLOG-<number>` or `NO-ISSUE` followed by a capital letter.
- [ ] PR body fills in Description and Reference, and sets a milestone.
