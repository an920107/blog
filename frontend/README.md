# Frontend

## Development

### Prerequisites

- [Bun](https://bun.sh/) - Package manager and runtime

### Setup

1. Install dependencies:

   ```bash
   bun install
   ```

2. Start development server:

   ```bash
   bun run dev
   ```

3. Build for production:

   ```bash
   bun run build
   ```

4. Preview production build:
   ```bash
   bun run preview
   ```

### Development Commands

- **Type checking**: `bun run --bun check`
- **Type checking (watch mode)**: `bun run --bun check:watch`
- **Linting**: `bun run --bun lint`
- **Formatting**: `bun run format`

### Project Structure

The frontend is built with SvelteKit and Tailwind CSS following Clean Architecture principles:

- `src/lib/` - Core application modules organized by feature
- `src/routes/` - SvelteKit route pages
- `src/app.html` - Main HTML template
- `src/app.css` - Global styles

Each feature module in `src/lib/` follows the Clean Architecture pattern:

- `domain/` - Business logic and entities
- `application/` - Use cases and application services
- `adapter/` - Interface adapters (presenters, view models)
- `framework/` - External frameworks (UI components, API services)

### Technology Stack

- **Framework**: SvelteKit 5
- **Styling**: Tailwind CSS 4
- **UI Components**: bits-ui, Lucide icons
- **Type Safety**: TypeScript
- **Linting**: ESLint with Prettier
- **Package Manager**: bun

### Sitemap & Feed

The frontend serves its own discovery documents, so the public URL scheme is defined only here:

- `/sitemap.xml` - every published post plus the static pages the backend cannot know about
  (home, post list, terms).
- `/feed.xml` - an RSS 2.0 feed of the 20 most recent posts. Items are identified by the post's
  semantic id, so their `guid` stays stable when URLs change.

Both are generated on demand from `src/lib/seo/` and served with `Cache-Control: public, max-age=300`,
so a newly published post shows up without a rebuild or a separate generation step.
