---
name: create-pr
description: "Create a pull request on this repository's Gitea instance with the gt CLI, following .gitea/PULL_REQUEST_TEMPLATE.yaml (Description, Package Changes, Screenshots, Reference, Checklist) and the CI PR title check. Use when asked to open, submit, or draft a pull request, or to fill in a PR description. Every gt and git push command must run with the terminal's unsandboxed permission."
---

# Create a Pull Request with gt

Use this skill when the user asks to open or submit a pull request for `squid/blog`
on `git.squidspirit.com`. For general CLI behaviour (authentication, repository
discovery, JSON fields, exit codes), the global `gt` skill applies; this skill adds
this repository's PR title, body, and milestone requirements.

`gt` is the official Gitea CLI and is what this machine has installed. Do not reach
for `tea`: it is not installed here, and a stale mise shim makes it fail with
`tea is not a valid shim`.

## Non-negotiables

1. **Title** must satisfy `.gitea/workflows/pr-title-check.yaml`:
   `^(BLOG-[0-9]+|NO-ISSUE) [A-Z].+$` — ticket prefix, one space, then a summary
   whose first character is uppercase.
2. **Body** must contain these sections, in this order, using `###` headings (this is
   what existing PRs in this repo use): `### Description`, `### Package Changes`,
   `### Screenshots`, `### Reference`, `### Checklist`. Write `None` in a section
   that has nothing to report.
3. **A milestone must be set** (`-m <title>`).
4. **Reference** must link the issue with `Resolves #<n>`, or be `None` for
   `NO-ISSUE` work.

## Sandbox and permission rules

The terminal may run inside Zed's sandbox. That sandbox blocks network access and
cannot write Git metadata. Therefore:

- Run **every** `gt ...` command with `unsandboxed: true` and a `reason` (for example:
  "gt needs network access to git.squidspirit.com and the stored login").
- Run `git push` (and anything else that writes `.git`) with `unsandboxed: true`.
- Always pass `--no-input`, so gt never opens a prompt or an editor. Without a
  terminal, an unconfirmed prompt fails with exit code 2.
- Pass `--yes` only for the one confirmation the user already approved — normally the
  push of a branch that is not on the remote yet. Never pass `--yes` to skip a
  question the user has not answered.
- Permission-protected commands reject shell substitution: no `$VAR`, `$(...)`,
  backticks, `$((...))`, `<(...)`. Never build the body with
  `--body "$(cat file)"`; use `-F <path>`.
- Do not retry a failing sandboxed `gt` call as a network glitch. Re-run it
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
| Milestone | Required. List open milestones with `gt milestone list -R squid/blog --no-input --json id,title,state` |
| Related issue | Issue number(s) for `Resolves #n`; `None` when there is none |
| Description points | What changed and why |
| Screenshots | Only when the UI changed |

Derive what you can (current branch, candidate milestones, the issue behind the
ticket) and confirm it. Show the final title, base, head, milestone, and rendered body
once before creating the PR, unless the user already approved that exact content in
this turn.

### 2. Pre-flight checks

```sh
git --no-pager status --short
git --no-pager branch --show-current
git --no-pager log --oneline origin/main..HEAD
gt auth status --no-input --json
gt repo view -R squid/blog --no-input --json full_name
gt pr list -R squid/blog --state open -L 30 --no-input --json number,title,head
```

- The working tree must be clean. If it is not, stop and ask; commits follow the
  `git-conventions` skill.
- The branch name must follow `git-conventions` (`BLOG-xxx_purpose` or
  `NO-ISSUE_purpose`). Rename before pushing if it does not.
- The head branch must not be the base branch. If it is, stop and ask.
- `gt auth status --json` exits 0 even when the credential is bad: read
  `credential_available` and `verified`. Missing or rejected credentials exit 4.
- An open PR for the same head branch already exists → stop and ask whether to update
  it instead (`gt pr edit <n>`).
- Compare against the actual base branch. The examples below assume `main`; replace
  `origin/main` and `-B main` with the chosen base branch.

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
- no manifest changes, or a formatting-only change (for example a `chore: format toml`
  commit that reflows a dependency table) → write `None` under `### Package Changes`

Lockfile-only updates (`bun.lock`, `Cargo.lock`) do not go in the table; mention them
in Description only if it matters.

### 4. Write the PR body

Copy `templates/pr-body.md` to `.pr-body.md` at the repository root with `write_file`,
fill every section, then hand the file to gt. Replace every placeholder, including the
example package row; if there are no dependency changes, replace the whole table with
`None`. Do not build the body with heredocs or shell substitution.

- **Description**: one bullet per change, plus the reason. Include behaviour changes,
  migrations, and deployment notes.
- **Package Changes**: the table, or `None`.
- **Screenshots**: `None` unless the UI changed. gt can attach files with
  `--attachment <path>`, but prefer leaving images to the user if they have not
  provided them.
- **Reference**: `Resolves #<n>` (multiple: `Resolves #1, #2`), or `None` for
  `NO-ISSUE`.
- **Checklist**: tick only what is true.
  - `A milestone is set` → `[x]` when `-m` is passed.
  - `The related issues has been linked to this branch` → `[x]` when Reference
    resolves an issue, otherwise `[ ]`.
- Do not add extra sections. If your agent guidance requires an attribution footer,
  put it after the checklist as a separate `---` block; otherwise leave it out.

### 5. Create the PR

```sh
gt pr create -R squid/blog --no-input \
  -t "BLOG-320 Leave more space between posts list and footer" \
  -B main -H BLOG-320_leave_more_space_between_posts_list_and_footer \
  -m 0.9 -F .pr-body.md
```

Run this unsandboxed.

- Optional flags: `-l/--label`, `-a/--assignee`, `-r/--reviewer`, `--reviewer-team`,
  `-p/--project`, `--draft`, `--attachment`.
- If the head branch is not on the remote, gt pushes it and asks for confirmation
  first; add `--yes` only after the user agreed to the push.
- Delete `.pr-body.md` once the command succeeds.

### 6. Verify and fix

```sh
gt pr view <number> -R squid/blog --no-input --json number,url,title,base,head,milestone,labels,state
gt pr checks <number> -R squid/blog --no-input --watch --json name,state
```

Check the title, base, head, and milestone, and confirm the body rendered from the
file. Fix problems with gt instead of asking the user:

```sh
gt pr edit <number> -R squid/blog --no-input -F .pr-body.md
gt pr edit <number> -R squid/blog --no-input -m 0.9
gt pr edit <number> -R squid/blog --no-input -t "BLOG-320 Corrected title"
```

Recreate `.pr-body.md` with `write_file` first if it was deleted. All of these run
unsandboxed. Report a red check instead of merging anything.

### 7. Report

Give the user the PR URL and number, title, `base ← head`, milestone, the Reference
line, and the final CI state. List anything they must finish manually (screenshots,
issue links, labels, reviewers).

## Failure handling

- `gt` missing or a broken shim → `gt --version`. If that fails, tell the user to
  install the Gitea CLI and stop. Do not fall back to `tea`.
- Authentication errors (exit 4, or `credential_available: false`) →
  `gt auth status --no-input --json`, then ask the user to run `gt auth login`. Never
  paste tokens into the PR body or command output.
- Repository inference picks the wrong repo (gt prefers `upstream` over `origin`) →
  always pass `-R squid/blog`.
- `head branch not found` → push the branch, then retry.
- Title check failure → fix it with `gt pr edit --title`, keeping the uppercase rule.
- `milestone not found` → run `gt milestone list -R squid/blog --no-input --json id,title`
  and use the exact milestone title or ID.
