# Blog

## Description

- Frontend: SvelteKit with Tailwind CSS
- Backend: Rust actix-web

Despite SvelteKit being a full-stack framework, I still decided to adopt a separate front-end and back-end architecture for this blog project. I believe that this separation makes the project cleaner, reduces coupling, and aligns with modern development practices. Furthermore, I wanted to practice developing a purely back-end API.

As for the more detailed development approach, I plan to use Clean Architecture for the overall structure. Of course, such a small project may not necessarily require such complex design patterns, but I want to give myself an opportunity to practice them.

These will allow me to become more proficient in these modern development practices and leave a lot of flexibility and room for adjustments in the future.

For more information about the development process, you can check out the [project board](https://git.squidspirit.com/squid/blog/projects). As for the details of the architecture and convention, you can find them in the [wiki](https://git.squidspirit.com/squid/blog/wiki).

## Build & Development Setup

### Prerequisites

- [Bun](https://bun.sh/) - Package manager and runtime for frontend
- [Rust](https://rustup.rs/) - For backend development
- [Podman](https://podman.io/) (Optional) - For containerized deployment

### Pre-commit Setup

This project uses pre-commit hooks to ensure code quality. To set up pre-commit:

1. Install pre-commit:

   ```bash
   pip install pre-commit
   ```

2. Install the git hook scripts:
   ```bash
   pre-commit install
   ```

The pre-commit configuration will automatically run:

- Backend Rust code checking
- Frontend linting
- SQL schema preparation

### Backend Setup

For detailed backend development setup, see [backend/README.md](./backend/README.md).

Quick start:

1. Install sqlx CLI: `cargo install sqlx-cli`
2. Run database migrations: `sqlx migrate run`
3. Prepare SQL schema: `cargo sqlx prepare --workspace`
4. Run the server: `RUST_LOG=debug cargo run`

### Frontend Setup

For detailed frontend development setup, see [frontend/README.md](./frontend/README.md).

Quick start:

1. Navigate to frontend directory: `cd frontend`
2. Install dependencies: `bun install`
3. Start development server: `bun run dev`
4. Build for production: `bun run build`

### Full Project Setup

To set up the entire project:

1. Clone the repository
2. Set up pre-commit hooks (see above)
3. Set up backend (see [backend/README.md](./backend/README.md))
4. Set up frontend (see [frontend/README.md](./frontend/README.md))
5. Start both servers for full-stack development

### MCP Server Setup

To configure and run the MCP (Model Context Protocol) servers for AI-assisted development:

1. **Install Go**: Ensure Go is installed on your system to run Go-based MCP servers (such as the Gitea MCP server).

2. **Install Bun**: Ensure Bun is installed on your system to run Svelte-based MCP servers (such as the Svelte MCP server via `bunx`).

3. **Install crates.io MCP**: Install `cratesio-mcp` using Cargo:

   ```bash
   cargo install cratesio-mcp
   ```

4. **Configure Environment Variables**: Export your Gitea access token:

   ```bash
   export GITEA_ACCESS_TOKEN="your_gitea_access_token_here"
   ```

## Deployment

Self-hosting uses the Kubernetes manifests in
[manifest/README.md](./manifest/README.md).

## License

The source code is licensed under the [Apache License 2.0](./LICENSE). You are free to use, modify, and redistribute the code, including for your own blog.

The project's name and branding ("SquidSpirit" / "魚之魷魂", the logo, and personal content) are **not** covered by the code license. See [TRADEMARK.md](./TRADEMARK.md) for the trademark policy — in short, if you deploy a modified copy, replace the branding with your own.
