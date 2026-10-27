---
name: create-pr
description: "Create a pull request on this repository's Gitea instance with the tea CLI, following .gitea/PULL_REQUEST_TEMPLATE.yaml (Description, Package Changes, Screenshots, Reference, Checklist) and the CI PR title check. Use when asked to open, submit, or draft a pull request, or to fill in a PR description. All tea and git push commands must run with the terminal's unsandboxed permission."
---

# Create a Pull Request with tea

Use this skill when the user asks to open or submit a pull request for
`squid/blog` on `git.squidspirit.com`. For general `tea` CLI behavior (login,
repository inference, fallbacks), the global `tea` skill applies; this skill adds
this repository's PR title, body, and milestone requirements.

## Non-negotiables

1. **Title** must satisfy `.gitea/workflows/pr-title-check.yaml`:
   `^(BLOG-[0-9]+|NO-ISSUE) [A-Z].+$` — ticket prefix, one space, then a
   summary whose first character is uppercase.
2. **Body** must contain these sections, in this order, using `###` headings
   (this is what existing PRs in this repo use):
   `### Description`, `### Package Changes`, `### Screenshots`,
   `### Reference`, `### Checklist`.
   Write `None` in a section that has nothing to report.
3. **A milestone must be set** (`--milestone`).
4. **Reference** must link the issue with `Resolves #<n>`, or be `None` for
   `NO-ISSUE` work.

## Sandbox and permission rules

The terminal may run inside Zed's sandbox. That sandbox blocks network access,
may hide `tea` from `PATH`, and cannot write Git metadata. Therefore:

- Run **every** `tea ...` command with `unsandboxed: true` and a `reason`
  (for example: "tea needs network access to git.squidspirit.com and the stored
  login").
- Run `git push` (and anything else that writes `.git`) with
  `unsandboxed: true`.
- Permission-protected commands reject shell substitution: no `$VAR`,
  `$(...)`, backticks, `$((...))`, `<(...)`. Never build the body with
  `--description "$(cat file)"`; use `--description-file <path>`.
- Do not retry a failing sandboxed `tea` call as a network glitch. Re-run it
  unsandboxed.

## Workflow

### 1. Collect inputs

Ask the user in a single message; do not guess:

| Input | Notes |
| --- | --- |
| Ticket | `BLOG-<number>` or `NO-ISSUE` |
| Title summary | Text after the prefix; start with an uppercase letter, keep it short |
| Base branch | Default `main`; release branches such as `release/0.8` exist |
| Head branch | Default the current branch; it must exist on `origin` |
| Milestone | Required. List open milestones with `tea milestones list -r squid/blog` |
| Related issue | Issue number(s) for `Resolves #n`; `None` when there is none |
| Description points | What changed and why |
| Screenshots | Only when the UI changed |

Derive what you can (current branch, candidate milestones) and confirm it. Show
the final title, base, head, milestone, and rendered body once before creating
the PR, unless the user already approved that exact content in this turn.

### 2. Pre-flight checks

```sh
git --no-pager status --short
git --no-pager branch --show-current
git --no-pager log --oneline origin/main..HEAD
```

- The working tree must be clean. If it is not, stop and ask; commits follow the
  `git-conventions` skill.
- The branch name must follow `git-conventions` (`BLOG-xxx_purpose` or
  `NO-ISSUE_purpose`). Rename before pushing if it does not.
- The head branch must not be the base branch. If it is, stop and ask.
- Compare against the actual base branch. The examples below assume `main`;
  replace `origin/main` and `--base main` with the chosen base branch.

Push the branch if `origin/<head>` is missing or behind:

```sh
git push -u origin HEAD
```

Run this unsandboxed.

### 3. Build the Package Changes table

Only dependency changes declared in manifests count:

```sh
git --no-pager diff --name-only origin/main...HEAD -- '*package.json' '*Cargo.toml'
git --no-pager diff origin/main...HEAD -- '*package.json' '*Cargo.toml'
```

Map each changed dependency from the diff:

- only a `+` line → `Added`
- only a `-` line → `Removed`
- both lines → `Updated`, with `old -> new` as the version
- no manifest changes → write `None` under `### Package Changes`

Lockfile-only updates (`bun.lock`, `Cargo.lock`) do not go in the table; mention
them in Description only if it matters.

### 4. Write the PR body

Copy `templates/pr-body.md` to `.pr-body.md` at the repository root with
`write_file`, fill every section, then hand the file to tea. Replace every
placeholder, including the example package row; if there are no dependency
changes, replace the whole table with `None`. Do not build the body with
heredocs or shell substitution.

- **Description**: one bullet per change, plus the reason. Include behavior
  changes, migrations, and deployment notes.
- **Package Changes**: the table, or `None`.
- **Screenshots**: `None` unless the UI changed. tea cannot upload attachments,
  so create the PR first and tell the user to attach images in the web UI, or
  put image URLs here.
- **Reference**: `Resolves #<n>` (multiple: `Resolves #1, #2`), or `None` for
  `NO-ISSUE`.
- **Checklist**: tick only what is true.
  - `A milestone is set` → `[x]` when `--milestone` is passed.
  - `The related issues has been linked to this branch` → `[x]` when Reference
    resolves an issue, otherwise `[ ]`.
- Do not add extra sections. If your agent guidance requires an attribution
  footer, put it after the checklist as a separate `---` block; otherwise leave
  it out.

### 5. Create the PR

```sh
tea pulls create -r squid/blog --title "BLOG-320 Leave more space between posts list and footer" --head BLOG-320_leave_more_space_between_posts_list_and_footer --base main --milestone 0.9 --description-file .pr-body.md
```

Run this unsandboxed.

- `--description-file` exists in tea 0.16.0. If `tea pulls create --help` does
  not list it, upgrade tea; only as a fallback pass `--description` with the
  body as one literal argument and avoid backticks.
- Optional flags: `--labels`, `--assignees`, `--allow-maintainer-edits`,
  `--draft`.
- Delete `.pr-body.md` once the command succeeds.

### 6. Verify and fix

```sh
tea pulls list -r squid/blog --state open --fields index,title,head,milestone --limit 10
```

Check the new PR's title, head branch, and milestone. Open the URL printed by
`tea pulls create` to confirm the body. Fix problems with tea instead of asking
the user:

```sh
tea pulls edit <index> -r squid/blog --description-file .pr-body.md
tea pulls edit <index> -r squid/blog --milestone 0.9
tea pulls edit <index> -r squid/blog --title "BLOG-320 Corrected title"
```

Recreate `.pr-body.md` with `write_file` first if it was deleted. All of these
run unsandboxed.

### 7. Report

Give the user the PR URL and number, title, `base ← head`, milestone, and the
Reference line. List anything they must finish manually (screenshots, issue
links, labels, reviewers).

## Failure handling

- `tea: command not found` → run unsandboxed first. If it is still missing, tell
  the user to install tea and stop.
- Authentication errors → `tea logins list`, then ask the user to run
  `tea logins add`. Never paste tokens into the PR body or command output.
- Repository inference errors → pass `-r squid/blog`.
- `head branch not found` → push the branch, then retry.
- Title check failure → fix it with `tea pulls edit --title`, keeping the
  uppercase rule.
- `milestone not found` → run `tea milestones list -r squid/blog` and use the
  exact milestone title.
