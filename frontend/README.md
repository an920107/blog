# Frontend

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version recommended)
- [pnpm](https://pnpm.io/) - Package manager

### Setup

1. Install dependencies:

   ```bash
   pnpm install
   ```

2. Start development server:

   ```bash
   pnpm dev
   ```

3. Build for production:

   ```bash
   pnpm build
   ```

4. Preview production build:
   ```bash
   pnpm preview
   ```

### Development Commands

- **Type checking**: `pnpm check`
- **Type checking (watch mode)**: `pnpm check:watch`
- **Linting**: `pnpm lint`
- **Formatting**: `pnpm format`

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
- **Package Manager**: pnpm
