# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

fs-chaot is a Pokemon card collection tracker built with Dioxus (Rust fullstack framework). The application allows users to:

- Search for Pokemon by ID or name
- View card details (sprite, names, rarity, book/page/side/entry positions)
- Track their collection in a SQLite database with ownership status
- Browse their collection in a visual "book view" with two-page spreads (desktop) or single-page view (mobile)
- Toggle card ownership via modal dialogs accessible from any view

The app features a fullstack architecture with server-side functions and client-side routing.

## Development Commands

This project uses **pixi** for environment and dependency management. All commands should be run through pixi:

### Core Development

- `pixi run serve` - Start development server with hot reload (web platform)
- `pixi run test` - Run all tests
- `pixi run fmt` - Check code formatting (cargo, taplo, prettier)
- `pixi run lint` - Run linting (clippy + dx check)
- `pixi run check-all` - Run all checks (fmt, test, lint) - **use before commits/PRs**

### Formatting (Destructive)

- `pixi run fmt-write` - Auto-format all code (cargo fmt, taplo fmt, prettier, dx fmt)

### Production Build

- `pixi run -e prod build` - Build for production (release mode)
- `pixi run -e prod build-optimized` - Build with optimizations

### CI/CD

The GitHub Actions workflow runs `fmt`, `test`, and `lint` jobs in parallel on pull requests.

## Architecture

### Fullstack Structure

The app uses Dioxus's fullstack capabilities with clear client/server boundaries:

**Client-side (web feature):**

- Components in `src/components/` handle UI rendering and user interactions
- Uses Dioxus Router with routes defined in `src/main.rs` (`Route` enum)
- Global signals: `CARDS` (card collection), `IS_AUTHENTICATED` (login state)

**Server-side (server feature):**

- Backend functions in `src/backend.rs` marked with `#[server]` macro
- SQLite database operations (rusqlite, conditional on `server` feature)
- Password validation and card CRUD operations
- Key server functions:
  - `get_all_owned_cards_db()` - Returns HashMap<usize, Card> of all owned cards (O(1) lookups)
  - `update_card_db(card)` - Updates existing card in database (supports full Card updates)
  - `save_card_db(card)` - Inserts new card into database
  - `get_card_by_id_db(id)` / `get_card_by_name_db(name)` - Fetch cards from database
  - `get_card_by_id_remote(id)` / `get_card_by_name_remote(name)` - Fetch from PokeAPI

**Important:** Server functions use the `#[server]` macro and are only compiled when the `server` feature is enabled. They automatically create API endpoints that can be called from the client.

### Data Model

The core data structure is `Card` (`src/card.rs`) with these key types:

- `Index` - Pokemon ID (1-1025+), validates > 0
- `Book` - Calculated from index: `ceil(index / 576)`
- `Page` - Two types: `absolut()` and `relative()` (relative to book)
- `Side` - A or B (12 cards per side)
- `Entry` - Position within a page side (1-12)
- `Name` - Pokemon name (English/German via PokeAPI)
- `Rarity` - Card rarity enum (Common, Uncommon, Rare, etc.)

**Card positioning constants:**

- `CARDS_PER_BOOK = 576`
- `CARDS_PER_PAGE = 24`

### External API Integration

`src/pokeapi.rs` handles all PokeAPI interactions:

- Fetches Pokemon data from `https://pokeapi.co/api/v2/pokemon/`
- Retrieves multilingual names from GitHub CSV
- Implements name override system via `pokemon_name_overrides.json` for edge cases
- Supports both ID-based and name-based lookups

### Component Architecture

Components are organized in `src/components/`:

**Core UI Components:**

- `login.rs` - Authentication UI
- `home.rs` - Main card browsing interface (SearchBar + CardContainer)
- `history.rs` - Collection history view (latest 12 cards with timestamps)
- `collection.rs` - Book view with ownership management (main collection feature)
- `search_bar.rs` - Pokemon search functionality (by ID or name)
- `nav_bar.rs` - Navigation bar with routing and logout
- `protected_route.rs` - Route protection wrapper for authenticated routes

**Card Display Components:**

- `card_view.rs` - Full card display for list views (Home, History)
- `card_view_compact.rs` - Compact card display for book view (3×4 grid)
- `placeholder_card.rs` - Placeholder for unowned cards in book view
- `card_container.rs` - Grid layout wrapper for multiple cards

**Shared Components:**

- `dialog.rs` - Modal dialog wrappers (DialogRoot, DialogContent, DialogTitle, DialogDescription)
- `book_navigation.rs` - Navigation controls for book view (arrows, dropdown, search)

**Routes:**

- `/` - Login page (public)
- `/home` - Search and browse cards (protected)
- `/history` - View recent card lookups (protected)
- `/collection` - Book view with ownership tracking (protected)

### Database Schema

SQLite schema (`src/backend.rs`):

```sql
CREATE TABLE cards (
    id INTEGER PRIMARY KEY,
    name_en TEXT NOT NULL,
    name_de TEXT NOT NULL,
    book INTEGER NOT NULL,
    page INTEGER NOT NULL,
    side TEXT NOT NULL,
    entry INTEGER NOT NULL,
    img_url TEXT NOT NULL,
    owned BOOLEAN NOT NULL CHECK (owned IN (0,1)),  -- Tracks collection ownership
    rarity TEXT NOT NULL,
    created_at DATETIME DEFAULT (datetime('now', 'localtime'))
);
```

**Database Operations:**

- **INSERT** - `save_card_db(card)` adds new cards (called when fetching from PokeAPI)
- **UPDATE** - `update_card_db(card)` modifies existing cards (used for ownership toggle)
- **SELECT** - Multiple functions for querying by ID, name, or fetching all owned cards
- **No DELETE** - Cards remain in database when marked as not owned (owned=0)

## Feature Flags

Defined in `Cargo.toml`:

- `web` - Web platform support (default)
- `desktop` - Desktop platform support
- `mobile` - Mobile platform support
- `server` - Server-side functionality + rusqlite dependency (default)

Default features: `["web", "server"]`

## Configuration Files

- `pixi.toml` - Dependency and task management, defines dev/prod environments
- `Cargo.toml` - Rust dependencies and feature flags
- `Dioxus.toml` - Dioxus framework configuration
- `clippy.toml` - Clippy lints for Dioxus-specific types (await-holding rules)
- `pokemon_name_overrides.json` - Manual name mappings for PokeAPI edge cases

## Environment Variables

- `APP_PASSWORD` - Required for authentication (server-side validation)

## Testing

The codebase has comprehensive unit tests:

- `src/card.rs` - Tests for all card positioning calculations (Index, Book, Page, Side, Entry)
- `src/pokeapi.rs` - Tests for CSV parsing and name lookup functions
- `src/backend.rs` - Includes an `#[ignore]` integration test that validates all 1025 Pokemon IDs

Run specific test: `cargo test <test_name> --features web,server`

Run ignored tests: `cargo test -- --ignored --features web,server`

## Collection Feature (Book View)

The collection view (`/collection` route) provides a visual book-style interface for browsing the entire Pokemon collection:

**Layout:**

- Desktop: Two-page spread (24 cards per page, 12 per side in 3×4 grid)
- Mobile: Single page view (24 cards in 3×8 grid)
- 43 total pages (1025+ Pokemon / 24 cards per page)

**Navigation:**

- Arrow buttons (◀ Previous / Next ▶) with disabled states at boundaries
- Page dropdown (1-43) for direct page jumps
- Search by Pokemon ID or name (only jumps to owned cards)

**Card Display:**

- Owned cards: Show sprite, German name (bold), English name (italic), ID, rarity
- Border colors indicate rarity: Gray (Common) → Cyan glow (Secret Rare)
- Placeholders: Show "?" for unowned cards with dashed border

**Ownership Management:**

- Click any card to open ownership dialog
- Dialog shows card details with "Add to Collection" / "Remove from Collection" button
- Ownership changes persist to database immediately
- Cards marked as not owned become placeholders (but stay in database with owned=0)

**State Management:**

- `owned_cards` signal uses HashMap<usize, Card> for O(1) lookups
- Loads all owned cards on mount via `get_all_owned_cards_db()`
- Updates local state optimistically after successful database operations

## Important Notes

- The production server URL is hardcoded in `src/main.rs` when `server` feature is disabled
- Database file location: `db/production.db`
- Sprite images use: `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/`
- When adding new components, update `src/components.rs` module exports
- All custom Rust types implement `ToSql`/`FromSql` for SQLite serialization when `server` feature is enabled
- Book view ignores the physical "Book" field from Card model - uses `Page::absolut()` for continuous page numbering
- CSS uses BEM-like naming convention with component-scoped classes (e.g., `.book-nav__button`)
- Responsive breakpoint: 768px (mobile vs desktop layout)
