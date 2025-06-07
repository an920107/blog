# Backend

## Development

### SQL Migration

1. Install sqlx

   ```bash
   cargo install sqlx-cli
   ```

2. Run migration

   ```bash
   sqlx migrate run
   ```

### Run Project

1. Prepare for sql schema setup

   ```bash
   cargo sqlx prepare --workspace
   ```

2. Run the server

   ```bash
   RUST_LOG=debug cargo run
   ```

3. (Optional) Hot restart

   1. Install `watchexec`

   2. Run the server with `watchexec`

      ```bash
      RUST_LOG=debug watchexec -e rs -r 'cargo run'
      ```

