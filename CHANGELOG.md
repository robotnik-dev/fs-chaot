# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-07

### Added

#### Collection Feature (Book View)

- **Book View Interface** - Visual collection browser at `/collection` route
  - Desktop: Two-page spread layout (12 cards per page in 3×4 grid)
  - Mobile: Single page view (24 cards in 3×8 grid)
  - 43 total pages covering all 1025+ Pokemon
- **Card Ownership Management** - Click any card to toggle ownership status
  - Modal dialog for adding/removing cards from collection
  - Displays card sprite, German name (emphasized), English name, ID, and rarity
  - Ownership persists to SQLite database
- **Navigation Controls**
  - Arrow buttons (Previous/Next) with disabled states at boundaries
  - Page dropdown for direct jumps (1-43)
  - Search by Pokemon ID or name (jumps to page if card is owned)
- **Visual Design**
  - Rarity-based border colors (Common → Secret Rare with glow effects)
  - Placeholder cards with "?" icon for unowned Pokemon
  - Responsive design with 768px breakpoint

#### Backend

- Added `get_all_owned_cards_db()` server function - Returns HashMap for O(1) ownership lookups
- Added `update_card_db(card)` server function - Updates existing cards in database
- Added `HashMap` import for efficient card lookups

#### Components

- Created `card_view_compact.rs` - Compact card display for book view
- Created `placeholder_card.rs` - Placeholder component for unowned cards
- Created `book_navigation.rs` - Navigation controls (arrows, dropdown, search)
- Implemented `collection.rs` - Main book view with state management (301 lines)

#### Styling

- Added ~450 lines of CSS for book view
  - Book navigation styles (search, buttons, dropdown)
  - Book page grid layouts (desktop 4×3, mobile 3×8)
  - Card compact styles with rarity-based borders
  - Dialog styles for ownership modal
  - Dark mode support
  - Mobile responsive breakpoints

### Changed

- Updated `CLAUDE.md` with comprehensive collection feature documentation
- Updated component exports in `src/components.rs` (alphabetically sorted)

### Technical

- Upgraded Dioxus from 0.7.0 to 0.7.1
- Database schema supports ownership tracking via `owned` BOOLEAN field
- State management uses HashMap for O(1) card lookups instead of Vec iteration
- Uses `Page::absolut()` for continuous page numbering (ignores physical "Book" field)
- CSS follows BEM-like naming convention (e.g., `.book-nav__button`)

[0.1.0]: https://github.com/robotnik-dev/fs-chaot/releases/tag/v0.1.0
