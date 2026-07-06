# Product

## Register

product

## Users

Developers and solo operators managing personal task lists. They work fast, scan rather than read, and want the interface to disappear into the task. Context: a quick capture during work, a batch review at end of day.

## Product Purpose

A personal todo manager: create, complete, edit, and delete tasks with search, status filtering, and pagination. Success is zero friction between thought and captured task; the UI should never be the bottleneck.

## Brand Personality

Calm, clinical, restrained. No hype, no decoration for its own sake. Every element earns its place by serving a task. Familiar over surprising. Linear / Notion / Things 3 in spirit.

## Anti-references

- SaaS clichés: purple/indigo gradients, glassmorphism, hero-metric templates.
- The 2026 AI cream/sand/beige default background.
- Identical icon+heading+text card grids repeated endlessly.
- Tiny uppercase tracked eyebrow text above every section.
- Over-rounded cards (border-radius 32px+). Ghost-card pattern (border + wide soft shadow).

## Design Principles

- **The tool disappears into the task.** Familiarity is the feature. No invented affordances for standard tasks.
- **Restraint over decoration.** One accent color for primary actions and current state. Neutral surfaces elsewhere.
- **Density without noise.** Show what matters, hide what doesn't. Tight type scale, consistent spacing.
- **State is always legible.** Every interactive element has default, hover, focus, active, disabled, loading, error.
- **Skeletons, not spinners.** Empty states teach, they don't apologize.

## Accessibility & Inclusion

- WCAG 2.1 AA contrast (body text ≥4.5:1, large text ≥3:1).
- Full keyboard navigation: tab order, focus rings, Enter to submit, Esc to cancel inline edit.
- `prefers-reduced-motion` respected on all transitions.
- Form errors announced; no color-only state signaling.
