# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Tech Stack

| Layer | Technology |
|---|---|
| Framework | Tauri v2 |
| Frontend | Svelte 5 + TypeScript |
| Styling | shadcn-svelte (nova) + Tailwind CSS v4 |
| Backend | Rust (2021 edition) |
| Git Engine | git2-rs (libgit2 bindings) |
| Monorepo | pnpm workspaces |

## What This Project Is

GitEase is a cross-platform desktop Git client built with Tauri v2. The data flow is:

```
Svelte UI → Tauri IPC (invoke) → Rust commands → git-core → libgit2
```

## Monorepo Structure

- `apps/desktop/frontend/` — SvelteKit + Svelte 5 frontend (runes mode enforced)
- `apps/desktop/src-tauri/` — Tauri v2 Rust backend (thin IPC layer only — no logic here)
- `packages/git-core/` — Rust library crate with all Git logic via `git2-rs`
- Root `Cargo.toml` — Cargo workspace covering `packages/git-core` and `apps/desktop/src-tauri`

## Commands

### Desktop app
Run from `apps/desktop/`:
```bash
pnpm dev        # starts Vite dev server + Tauri window
pnpm build      # production build
```

### Frontend only
Run from `apps/desktop/frontend/`:
```bash
pnpm dev        # Vite dev server at http://localhost:5173
pnpm check      # svelte-check type checking
pnpm build      # static build output → build/
```

### Rust workspace
Run from repo root:
```bash
cargo check                  # check all crates
cargo check --bin gitease    # check Tauri binary only
cargo test -p git-core       # run git-core tests
```

## Key Architectural Rules

**Adding a new Git operation:**
1. Add the function to `packages/git-core/src/lib.rs` — pure Rust, no Tauri imports
2. Add a `#[tauri::command]` wrapper in `apps/desktop/src-tauri/src/main.rs`
3. Register it in `tauri::generate_handler![]` in `main()`
4. Call it from Svelte with `invoke('command_name', { argName: value })`

**git-core must stay Tauri-free.** It's a standalone library crate. All Tauri-specific code belongs in `src-tauri`.

**Svelte 5 runes are enforced** — `svelte.config.js` sets `runes: true` for all non-`node_modules` files. Use `$state`, `$props`, `$derived`, `$effect` — not the legacy store/reactive syntax.

**Frontend is static (no SSR).** `src/routes/+layout.ts` sets `ssr = false` and `prerender = true` because Tauri loads the app as a local file bundle. Never re-enable SSR.

## Frontend Stack Details

- **Tailwind v4** via `@tailwindcss/vite` plugin (no `tailwind.config.js` — config lives in `src/app.css` via `@theme`)
- **shadcn-svelte** style `nova`, components live in `src/lib/components/ui/`
- **`cn()` utility** in `src/lib/utils.ts` — always use for conditional Tailwind classes
- Adding shadcn components: `pnpm dlx shadcn-svelte@latest add <component>` from `apps/desktop/frontend/`

## Rust Crate Versions

- `tauri = "2"`, `git2 = "0.20"`, `serde = "1"`, `thiserror = "2"`
- `tauri-build = "2"` in `[build-dependencies]`
- Cargo edition `2021` on both crates
