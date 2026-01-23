# AGENTS.md

This file provides guidelines for agentic coding agents working on the ZTVRUI project.

## Project Overview

ZTVRUI is a ZeroTier One Controller WebUI with:
- **Backend**: Rust (Axum framework)
- **Frontend**: Vue 3 + TypeScript + Tailwind CSS
- **Database**: None (uses ZeroTier API directly)

## Build Commands

### Backend (Rust)
```bash
cd backend
# Build
cargo build

# Build with optimizations (release)
cargo build --release

# Check for compilation errors
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Run tests (if any exist)
cargo test

# Run a specific test
cargo test test_name

# Clean build artifacts
cargo clean
```

### Frontend (Vue + TypeScript)
```bash
cd frontend

# Install dependencies
pnpm install

# Development server
pnpm dev

# Build for production
pnpm build

# Type checking
pnpm type-check

# Linting
pnpm lint

# Lint ESLint only
pnpm lint:eslint

# Lint Oxlint only
pnpm lint:oxlint

# Format code with Prettier
pnpm format

# Preview production build
pnpm preview
```

## Linting & Code Style

### Frontend Code Style
- **Framework**: Vue 3 Composition API with TypeScript
- **Styling**: Tailwind CSS utility classes
- **Component libraries**: Reka UI components
- **State management**: Pinia stores
- **Routing**: Vue Router 4
- **HTTP client**: Axios

#### TypeScript/JavaScript
- Use TypeScript strict mode
- Prefer explicit type annotations
- Use `interface` for object shapes, `type` for unions/tuples
- Use arrow functions for methods and callbacks
- Use `const` for variables that won't be reassigned
- Use template literals for string interpolation

#### Vue Components
- Use PascalCase for component names (multi-word required except for UI components)
- Use Composition API (`<script setup>` syntax preferred)
- Use reactive state with `ref()` and `computed()`
- Use lifecycle hooks from `@vueuse/core` when available
- Component structure: template → script → style
- Use single-file components (.vue files)

#### Imports
- Group imports: Vue/third-party → local modules
- Use absolute imports with `@/` alias
- Remove unused imports (oxlint enforces this)
- Use named imports for better tree-shaking

#### Error Handling
- Use try/catch for async operations
- Display user-friendly error messages
- Log errors to console in development
- Use global error handling in Axios interceptors

### Backend Code Style (Rust)
- **Framework**: Axum 0.8
- **Error handling**: `thiserror` crate with custom `AppError` enum
- **Logging**: `tracing` crate
- **Serialization**: `serde` with JSON
- **Async runtime**: Tokio

#### Code Organization
- Module structure: `mod error; mod handlers; mod middleware; mod routes; mod services;`
- Use `Result<T>` alias from error module (`crate::error::Result<T>`)
- Services contain business logic
- Handlers map HTTP requests to service calls
- Middleware for authentication, rate limiting, etc.

#### Error Handling
- Use `AppError` enum for all errors
- Implement `IntoResponse` for `AppError`
- Use `?` operator for error propagation
- Return appropriate HTTP status codes (401, 429, 500, 502, etc.)

#### Naming Conventions
- Use snake_case for variables, functions, modules
- Use PascalCase for types, traits, enums
- Use SCREAMING_SNAKE_CASE for constants
- Use descriptive names that indicate purpose

#### Imports & Modules
- Group imports: std → external crates → internal modules
- Use absolute paths within crate: `crate::error::AppError`
- Re-export commonly used types from module `mod.rs` files

## Testing

### Current Test Status
- **Frontend**: No test framework configured
- **Backend**: No test framework configured
- **CI Pipeline**: Build and lint checks only

### Running Tests
When tests are added:
```bash
# Frontend (when configured)
cd frontend
pnpm test

# Backend
cd backend
cargo test

# Run specific test
cargo test test_function_name
```

## Development Workflow

### Setup
1. Install Rust toolchain for backend
2. Install Node.js + pnpm for frontend
3. Configure ZeroTier token in `config.json`
4. Build frontend and copy to backend/dist
5. Run backend server

### CI/CD Pipeline
- GitHub Actions run on push to main
- Frontend: lint → type-check → build
- Backend: build with frontend artifacts
- Artifacts uploaded for releases

## Project Structure

```
/
├── backend/          # Rust backend
│   ├── src/
│   │   ├── error/   # Error types and handling
│   │   ├── handlers/# HTTP request handlers
│   │   ├── middleware/# Auth, rate limiting
│   │   ├── routes/  # Route definitions
│   │   ├── services/# Business logic
│   │   ├── models/  # Data structures
│   │   ├── state/   # Application state
│   │   └── utils/   # Utility functions
│   └── Cargo.toml
├── frontend/         # Vue frontend
│   ├── src/
│   │   ├── components/# Vue components
│   │   │   └── ui/  # Reka UI components
│   │   ├── stores/  # Pinia stores
│   │   ├── types/   # TypeScript types
│   │   ├── router/  # Vue Router
│   │   └── utils/   # Utility functions
│   └── package.json
└── docs/            # Documentation
```

## Important Notes for Agents

### Code Quality
- Always run `pnpm lint` and `pnpm type-check` for frontend changes
- Always run `cargo fmt` and `cargo clippy` for backend changes
- Check CI passes before considering work complete

### Security
- Never hardcode credentials or tokens
- Validate all user input on backend
- Use proper authentication middleware
- Sanitize API responses

### Performance
- Frontend: Use Vue's reactivity system efficiently
- Backend: Use async/await for I/O operations
- Minimize dependencies when possible

### Documentation
- Update README.md for user-facing changes
- Add code comments for complex logic
- Document API changes if applicable

## Common Patterns

### Frontend API Calls
```typescript
// Use Axios instance from src/utils/request.ts
import { request } from '@/utils/request'

async function fetchData() {
  try {
    const response = await request.get('/api/endpoint')
    return response.data
  } catch (error) {
    console.error('API call failed:', error)
    throw error
  }
}
```

### Backend Handler
```rust
use axum::{extract::State, Json};
use crate::{services::SomeService, error::Result};

pub async fn handler(
  State(service): State<SomeService>,
) -> Result<Json<ResponseData>> {
  let data = service.get_data().await?;
  Ok(Json(data))
}
```

### Error Propagation
```rust
// Use AppError and Result alias
use crate::error::{AppError, Result};

fn might_fail() -> Result<()> {
  let result = operation()?;  // Automatically converts to AppError
  Ok(())
}
```

## Troubleshooting

### Common Issues
1. **Frontend build fails**: Check TypeScript errors with `pnpm type-check`
2. **Backend won't compile**: Run `cargo check` for detailed errors
3. **Missing dependencies**: Run `pnpm install` or `cargo build`
4. **API errors**: Check ZeroTier token and network connectivity

### Debugging
- Frontend: Use Vue DevTools browser extension
- Backend: Set log level to "debug" in config
- Check browser console for frontend errors
- Check server logs for backend errors