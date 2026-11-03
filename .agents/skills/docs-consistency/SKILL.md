---
name: docs-consistency
description: "Keeps docs/ in step with the code. Use when a change touches anything docs/ describes — backend or frontend layers, crate or file layout, request flow, naming, error handling, commands, environment variables, or the commit/branch/PR rules — and when you notice docs/ contradicting the implementation. Decides case by case whether to update the document or to raise the difference with the user before changing anything."
---

# Keeping docs/ in Step with the Implementation

`docs/` is the human-readable description of this project; the code is what actually
runs. When they disagree, resolve it deliberately. Silent drift is the failure mode this
skill exists to prevent: a doc that confidently describes a layer, command, or rule that
no longer exists is worse than no doc.

## The documents

| File | Covers |
| --- | --- |
| `docs/architecture.md` | System shape (§1), repo layout (§2), the Clean Architecture layers shared by both stacks (§3), backend (§4), frontend (§5), coding conventions (§6), the add-a-feature checklist (§7), commands (§8) |
| `docs/git-convention.md` | Ticket prefixes (§1), commit messages (§2), branch names (§3), pull requests (§4), what CI enforces (§5) |

Related sources of truth that move with them:

- `backend/README.md`, `frontend/README.md`, `manifest/README.md` — setup and
  sub-project specifics. `README.md` holds the CI variables/secrets table.
- `.agents/skills/git-conventions/SKILL.md` — mirrors `docs/git-convention.md` for
  agents. **If a git rule changes, both must change in the same commit.**
- `.agents/skills/frontend-strings-links/SKILL.md` — the rule summarised in
  `architecture.md` §5.5.
- `backend/.env.example` and `frontend/.env.example` — the variables that
  `architecture.md` §4.8 points at.

## Decide: update the doc, or discuss first

The default is **update the doc**, because the doc describes the code you just changed.
Escalate to the user instead when any of the following is true:

1. **You would be rewriting a decision, not a mechanism.** §1 explains why the API is a
   separate service; §3 states the dependency rule and which layers may import what.
   Changing those is an architecture change, not a doc fix.
2. **The code and the doc contradict each other and you cannot tell which is intended.**
   The doc may be the specification and the code the bug. Then fix the code, and say so.
3. **The doc's claim is now "mostly" true.** If a rule gains exceptions, do not quietly
   soften the wording. Ask whether the rule should be relaxed or the code brought back in
   line. (Example: §6 states `unwrap()` is avoided in request paths and names the single
   OIDC exception — that is what an acknowledged exception looks like.)
4. **The edit is structural**: renaming or merging layers, moving crates, splitting a
   document, renumbering sections, or adding a new `.md` under `docs/`.
5. **The doc states something you cannot verify from the repository** — a deployment
   step, a CI secret, a manual server-side setting. Do not guess; ask.

Never rewrite a document to match a bug. If you believe the implementation is wrong,
fix the implementation and tell the user what the doc required.

When you defer, say exactly four things in one message: what the code does, what the doc
says, which one you think is intended, and the specific edit you would make. That way the
user can answer in one line.

## Update checklist

When you do update a document:

- Verify every claim against the code first. Do not write module, file, command, or
  setting names from memory — confirm the path exists before citing it.
- Match the existing voice: English prose, numbered `##` sections in
  `architecture.md`, tables for enumerations, relative links (`./git-convention.md`,
  `../backend/README.md`), lines wrapped near 95 columns.
- Keep the shared-layer framing. If you change one stack's layering or naming, check
  whether §3's cross-stack table and the other stack's section need the same edit.
- Update the index when the set of documents changes: `README.md` links into `docs/`, and
  §9 "See also" lists the related READMEs.
- Prefer a short accurate statement over a long stale one. If a subsection no longer
  earns its place, ask before deleting it.
- Ship the doc change with the code change — same commit, or the commit immediately
  after — and mention it in the PR `Description`. Do not leave it for a follow-up ticket
  unless the user asks for one.
- Do not add speculative content: no roadmaps, no "will be" sections, no patterns the
  code does not already use.

## Area → section map

Use this to find the paragraph that needs editing.

| Change you made | Check |
| --- | --- |
| Added/moved a backend crate or module file | §2, §4.1, §4.2, §7 |
| Added a use case, controller, handler, or route | §4.3, §4.7, §7 |
| Changed DI wiring or the container | §4.4 |
| Added an error variant or changed status-code mapping | §4.5 |
| Added a migration, renamed a table, refreshed `.sqlx` | §4.6, §6 "Data and naming across the boundary" |
| Added a frontend feature module, store, or composition wiring | §5.1, §5.2, §5.3, §7 |
| Changed routing, loaders, or SSR data handoff | §5.4, §5.3 (dehydrate/rehydrate) |
| Added UI text or a URL | §5.5 — and follow the `frontend-strings-links` skill |
| Changed styling or the shared UI kit | §5.6 |
| Changed naming, formatting, lint, or language conventions | §6 |
| Added/changed a script, command, hook, or CI step | §8, `git-convention.md` §5, and the affected sub-project README |
| Changed environment variables or configuration | §4.8 or `frontend/README.md`, plus the matching `.env.example` |
| Changed commit, branch, or PR rules | `git-convention.md` §1–§6 **and** the `git-conventions` skill |
| Changed deployment manifests or release flow | `manifest/README.md`, `README.md` "CI configuration" |

## Cheap drift checks

Before editing a section that makes a sweeping claim ("every handler…", "no layer
imports…"), confirm it still holds. Examples of the kind of check to run:

```sh
# handler count vs. files carrying a utoipa annotation
ls backend/feature/*/src/framework/web/*_handler.rs | wc -l
grep -rl "utoipa::path" backend/feature/*/src/framework/web/ | wc -l

# the route scopes §4.7 names
grep -rn "web::scope" backend/feature/*/src/framework/web/*_routes.rs

# inner layers must stay free of framework crates
grep -rln "actix\|sqlx\|serde" backend/feature/*/src/domain backend/feature/*/src/application

# where record types actually live
find backend -name "*_record.rs"

# tables and functions referenced in §4.6
grep -rn "RENAME TO" backend/migrations/
```

Treat a mismatch as a finding: it is either stale documentation or a violated
convention, and only the user can tell you which.

## Do not

- Do not create new documents under `docs/`, or restructure `docs/`, without being asked.
- Do not edit `docs/` purely to make a test or grep pass.
- Do not document a convention the code only follows in one place.
- Do not repeat document contents in your final message; summarise the section you
  changed and why.
