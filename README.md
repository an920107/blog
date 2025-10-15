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

- [Node.js](https://nodejs.org/) (LTS version recommended)
- [pnpm](https://pnpm.io/) - Package manager for frontend
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

- Backend Rust code checking and formatting
- Frontend linting and formatting
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
2. Install dependencies: `pnpm install`
3. Start development server: `pnpm dev`
4. Build for production: `pnpm build`

### Full Project Setup

To set up the entire project:

1. Clone the repository
2. Set up pre-commit hooks (see above)
3. Set up backend (see [backend/README.md](./backend/README.md))
4. Set up frontend (see [frontend/README.md](./frontend/README.md))
5. Start both servers for full-stack development

## License

This project uses a combination of the [MIT License and a custom license](./LICENSE.md). Based on the MIT License, anyone is permitted to use the code. However, before deploying the code, they must first replace any information belonging to "me" or any content that could identify "me," such as logos, names, and "about me" sections.
