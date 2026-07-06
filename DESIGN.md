# Design

> Captured from existing shadcn/ui tokens in `apps/web/src/styles.css`.
> Product register: calm, clinical, restrained. One accent for actions + state.

## Color strategy

**Restrained.** Tinted neutrals + one accent ≤10% of surface. Accent reserved for primary actions, current selection, and state indicators.

- Theme: light default, dark supported via `.dark`.
- All values OKLCH.
- Body: true near-white (chroma 0), not warm cream.

## Palette

### Light

| Token              | OKLCH                       | Use               |
| ------------------ | --------------------------- | ----------------- |
| background         | `oklch(1 0 0)`              | page surface      |
| foreground         | `oklch(0.145 0 0)`          | body ink          |
| card               | `oklch(1 0 0)`              | card surface      |
| popover            | `oklch(1 0 0)`              | dropdowns         |
| primary            | `oklch(0.205 0 0)`          | primary actions   |
| primary-foreground | `oklch(0.985 0 0)`          | text on primary   |
| secondary          | `oklch(0.97 0 0)`           | secondary surface |
| muted              | `oklch(0.97 0 0)`           | muted surface     |
| muted-foreground   | `oklch(0.556 0 0)`          | secondary text    |
| accent             | `oklch(0.97 0 0)`           | hover surface     |
| destructive        | `oklch(0.577 0.245 27.325)` | delete, errors    |
| border             | `oklch(0.922 0 0)`          | hairlines         |
| input              | `oklch(0.922 0 0)`          | input borders     |
| ring               | `oklch(0.708 0 0)`          | focus ring        |

### Dark

Mirrored in `.dark` — foreground/background invert, destructive warms to `oklch(0.704 0.191 22.216)`.

## Typography

- **Family:** Nunito (already loaded in `_layout.tsx`). One family carries headings, labels, body, data. Product register: no display pairing needed.
- **Scale** (fixed rem, ratio ~1.2): `3rem / 2rem / 1.25rem / 1rem / 0.875rem`.
- Body: 1rem (16px) minimum. Line-height 1.6 body, 1.1 headings.
- Headings: `tracking-tight`. Secondary text: `text-muted-foreground`.
- Line length: 65–75ch on prose columns.

## Spacing

Unified scale: **8 / 16 / 24 / 48 / 96px** (Tailwind `gap-2` / `gap-4` / `gap-6` / `gap-12` / `gap-24`).

- Section breaks: `gap-12` (48px)
- Row gaps: `gap-4` (16px)
- Form field groups: `gap-2` (8px)
- Spacing-driven grouping over decorative borders.

## Radius

`--radius: 0.625rem`. Cards/buttons: 6–10px. Tags/badges: full pill.

## Components

shadcn/ui (new-york style): Button, Input, Card, Badge, Select, Checkbox. Each has default/hover/focus/active/disabled/loading/error.

- Skeletons for loading, not spinners in content.
- Empty states that teach the interface.
- Consistent affordances: same button shape, same form-control vocabulary.

## Motion

- 150–250ms transitions. `transition-[color,box-shadow]` on inputs, `transition-all` on buttons.
- `prefers-reduced-motion: reduce` → instant/crossfade fallback.
- No orchestrated page-load sequences.

## Layout

- Max-width content container, centered. `main` holds the page surface.
- Responsive: structural (stack on mobile), not fluid type.
- Flexbox for 1D, Grid for 2D. `repeat(auto-fit, minmax(280px, 1fr))` for card grids.
