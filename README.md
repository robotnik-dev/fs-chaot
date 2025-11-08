# fs-chaot - Pokemon Card Collection Tracker

A fullstack Pokemon card collection tracker built with Dioxus and Rust.

## Developer Guide

### Prerequisites

This project uses **pixi** for environment and dependency management. All commands should be run through pixi.

Install pixi: https://pixi.sh/latest/

### Project Structure

```
fs-chaot/
├─ assets/              # Static assets (CSS, images)
├─ src/
│  ├─ main.rs          # Entry point, routing, global signals
│  ├─ backend.rs       # Server functions (SQLite operations)
│  ├─ card.rs          # Card data model and positioning logic
│  ├─ pokeapi.rs       # PokeAPI integration
│  ├─ components/      # UI components
│  │  ├─ collection.rs        # Book view (main collection feature)
│  │  ├─ card_view_compact.rs # Compact card display
│  │  ├─ book_navigation.rs   # Navigation controls
│  │  └─ ...
├─ db/                 # SQLite database
├─ Cargo.toml         # Rust dependencies and feature flags
├─ Dioxus.toml        # Dioxus configuration
├─ pixi.toml          # Pixi environment and tasks
└─ CLAUDE.md          # Detailed architecture documentation
```

### Development Commands

#### Core Development

```bash
# Start development server with hot reload
pixi run serve

# Run all tests
pixi run test

# Check code formatting (non-destructive)
pixi run fmt

# Run linting (clippy + dx check)
pixi run lint

# Run all checks (fmt, test, lint) - use before commits/PRs
pixi run check-all
```

#### Formatting (Destructive)

```bash
# Auto-format all code (cargo fmt, taplo fmt, prettier, dx fmt)
pixi run fmt-write
```

#### Production Build

```bash
# Build for production (release mode)
pixi run -e prod build

# Build with optimizations
pixi run -e prod build-optimized
```

### Environment Variables

Create a `.env` file in the project root:

```env
APP_PASSWORD=your_password_here
```

The dev server will automatically load environment variables from `.env`.

### First-Time Setup

1. Install pixi (if not already installed)
2. Clone the repository
3. Create `.env` file with `APP_PASSWORD`
4. Run `pixi run serve`
5. Navigate to `http://localhost:8080`

### Features

- **Search** - Find Pokemon by ID or name (PokeAPI integration)
- **Collection Tracking** - Mark cards as owned/unowned
- **Book View** - Visual collection browser with two-page spreads
- **History** - View recently looked up cards
- **Authentication** - Password-protected access

### Tech Stack

- **Framework**: Dioxus 0.7.1 (Rust fullstack)
- **Database**: SQLite with rusqlite
- **Styling**: CSS with custom properties
- **Build**: Pixi + Cargo
- **CI/CD**: GitHub Actions (fmt, test, lint in parallel)

### Contributing

1. Create a feature branch
2. Make changes
3. Run `pixi run check-all` to ensure all checks pass
4. Commit with descriptive message
5. Push and create PR

For detailed architecture information, see [CLAUDE.md](./CLAUDE.md).
