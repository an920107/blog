---
name: frontend-strings-links
description: Enforce this repository's rule that frontend UI strings live in `frontend/src/lib/strings.ts` and constant links/URLs live in `frontend/src/lib/links.ts`. Use when adding or editing user-facing text, aria-labels, placeholders, titles, or error messages, or any hardcoded URL, external link, mailto, or share intent in the SvelteKit frontend.
---

# Frontend Strings & Links Conventions

In this repository the SvelteKit frontend must not hardcode UI text or URLs in
components. They belong in two central files:

- `frontend/src/lib/strings.ts` — every user-facing string.
- `frontend/src/lib/links.ts` — every constant URL / link.

Components reference them through the `$lib` alias:

```ts
import { Strings } from '$lib/strings';
import { Links } from '$lib/links';
```

## When to add to `Strings`

Add a `Strings` constant for any user-facing text, including:

- Buttons, menu items, headings, table headers, labels
- `aria-label`, `title`, and `alt` text
- Placeholders, tooltips, hints, and empty-state messages
- Validation or error messages shown to the user

Do not leave literal user-facing strings inline in `.svelte` files.

### Format

`Strings` is an abstract class of `static readonly` members with explicit
`string` types and UPPER_SNAKE_CASE names:

```ts
export abstract class Strings {
	static readonly APP_NAME: string = '魚之魷魂 SquidSpirit';
	static readonly SHARE_TO_X: string = 'X';
	static readonly TOC: string = '章節目錄';
}
```

The main block is kept roughly alphabetical — insert new members in the matching
position instead of appending to the end.

## When to add to `Links`

Add a `Links` constant for any fixed URL or link: external sites, `mailto:`,
feeds, and share intents. Do not hardcode URLs in components.

`Links` is an abstract class of `static readonly` members. Two shapes are used:

1. Constant URLs, typed as `URL` and built with `new URL(...)`:

```ts
static readonly YOUTUBE: URL = new URL('https://www.youtube.com/@squidspirit16');
static readonly RSS: URL = new URL('feed.xml', Environment.APP_BASE_URL);
```

2. URL builders, for things that depend on a runtime value (such as a share
   intent). They take the target `URL` and return a `URL`:

```ts
static readonly FACEBOOK_SHARE: (target: URL) => URL = (target: URL) => {
	const url = new URL('https://www.facebook.com/sharer/sharer.php');
	url.searchParams.append('u', target.href);
	return url;
};
```

Build query strings with `URL` + `searchParams` (never manual concatenation) so
values are encoded correctly.

Values that depend on configuration come from `Environment` (`$lib/environment`).
Internal app routes are navigated with `resolve()` from `$app/paths`, not stored
in `Links`.

## Usage

```svelte
<script lang="ts">
	import { Links } from '$lib/links';
	import { Strings } from '$lib/strings';
</script>

<a href={Links.EMAIL.href} rel="external" aria-label={Strings.SHARE_TO_EMAIL}>
	<i class="fa-solid fa-envelope"></i>
</a>

<button aria-label={Strings.SHARE} onclick={shareNatively}>{Strings.SHARE}</button>
```

## Checklist

Before finishing a frontend change:

1. No new literal user-facing strings in `.svelte` files — they live in `Strings`.
2. No new hardcoded URLs — they live in `Links`.
3. New members use `static readonly`, an explicit type, and are inserted in the
   conventional (alphabetical) position.
4. Run `bun run lint && bun run check` inside `frontend/`.
