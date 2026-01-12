# api2spec-fixture-rocket

A Rocket (Rust) API fixture for testing api2spec framework detection and route extraction.

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

The server will start at `http://localhost:8000` by default.

## API Endpoints

### Health

- `GET /health` - Health check
- `GET /health/ready` - Readiness check

### Users

- `GET /users` - List all users
- `GET /users/<id>` - Get user by ID
- `POST /users` - Create a new user
- `PUT /users/<id>` - Update user by ID
- `DELETE /users/<id>` - Delete user by ID
- `GET /users/<user_id>/posts` - Get posts for a user

### Posts

- `GET /posts` - List all posts
- `GET /posts/<id>` - Get post by ID
- `POST /posts` - Create a new post

## Development

```bash
# Build in release mode
cargo build --release

# Run tests
cargo test

# Check for issues
cargo check
cargo clippy
```
