# Backend

## Development

### Prerequisites

- [Rust](https://rustup.rs/) - Latest stable version
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) - Database migration tool
- [watchexec](https://github.com/watchexec/watchexec) (Optional) - For hot reloading

### Setup

1. Install sqlx CLI:

   ```bash
   cargo install sqlx-cli
   ```

2. Run database migrations:

   ```bash
   sqlx migrate run
   ```

3. Prepare SQL schema:

   ```bash
   cargo sqlx prepare --workspace
   ```

4. Run the server:
   ```bash
   RUST_LOG=debug cargo run
   ```

### Development Commands

- **Run server**: `RUST_LOG=debug cargo run`
- **Hot reload** (optional): `RUST_LOG=debug watchexec -e rs -r 'cargo run'`
- **Database migration**: `sqlx migrate run`
- **Schema preparation**: `cargo sqlx prepare --workspace`
- **Build**: `cargo build`
- **Test**: `cargo test`

### Project Structure

The backend follows Clean Architecture principles with a modular structure:

- `server/` - Main server application and configuration
- `feature/` - Feature modules organized by domain
  - `auth/` - Authentication and authorization
  - `post/` - Blog post management
  - `label/` - Label/tag system
  - `image/` - Image handling
   - `search/` - Semantic search and RAG support
  - `common/` - Shared utilities and types
- `migrations/` - Database migration scripts

Each feature module follows the Clean Architecture pattern:

- `domain/` - Business logic and entities
- `application/` - Use cases and application services
- `adapter/` - Interface adapters (controllers, presenters)
- `framework/` - External frameworks (database, HTTP)

### Technology Stack

- **Framework**: Actix-web
- **Database**: PostgreSQL with SQLx
- **AI / Search**: RAG-based semantic search with embeddings and Qdrant
- **Authentication**: JWT-based
- **Serialization**: Serde
- **Migration**: SQLx migrations
- **Logging**: env_logger with RUST_LOG

### Search Embedding Cache

The search module supports Redis-based query embedding cache.

- `SEARCH_CACHE_KEY_PREFIX` - Redis key namespace prefix

Implementation details:

- Only query embedding is cached.
- Search result IDs are not cached.
- Cache uses existing `REDIS_URL`.
- Cache is enabled by default.
- Cache key model revision comes from embedding configuration.
- `index_post` does not reset query embedding cache because cache entries only depend on query string.
