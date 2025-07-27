# Codebase Overview

## Architecture

- **Backend**: Rust-based web server using Axum framework (`server/ws/`)
- **Frontend**: SolidJS application with Vite (`ui/web/`)
- **R&D**: Research and development documentation (`r&d/`)

## Backend (`server/ws/`)

- **Framework**: Axum web framework with async support
- **Database**: SQLite with SQLx for database operations
- **Features**: API endpoints, validation, error handling
- **Key modules**:
  - Package management (`src/package/`)
  - Boards functionality (`src/boards.rs`)
  - Database layer (`src/db/`)
  - API layer (`src/api/`)

## Frontend (`ui/web/`)

- **Framework**: SolidJS with TypeScript
- **Build Tool**: Vite
- **Styling**: TailwindCSS
- **Key features**:
  - Package management
  - Boards functionality  
