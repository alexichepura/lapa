# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Database schema migrations with [refinery](https://docs.rs/refinery),
  embedded in the `cli` binary and applied with `cargo cli migrate`
  (`db/migrations/`, `db` crate). `V1__init.sql` mirrors `schema.sql`, which is
  kept for Cornucopia codegen.

### Changed

- Admin image uploads (product and content) now send the file as a real
  `multipart/form-data` field through Leptos server functions
  (`input = MultipartFormData` / `MultipartData`), replacing the previous
  approach of stuffing a JSON-serialized byte array into a hidden input.
- Styling migrated from Sass (`style/main.scss`) to plain-CSS **CSS modules**
  with [`stylance`](https://github.com/basro/stylance-rs): component rules live
  in `*.module.css` files and are referenced through hashed class constants, the
  bundle is produced by `stylance` and inlined by cargo-leptos Lightning CSS
  bundling (`style-bundle = true`). No more `dart-sass` step.

### Fixed

- Site/admin SSR no longer returns 500 when the `Settings` record is missing or
  a DB call fails; `settings_db` falls back to defaults and `robots.txt`
  returns proper 404/500 responses.
- Rich-text rendering escapes link `href` (as a quoted attribute) and image
  `alt`/`caption`, and drops `javascript:`/`vbscript:`/`data:` hrefs.
- Rendering no longer panics on a non-UTF-8 `User-Agent` header.
- Admin dashboard stats work again: each site page render is recorded in the
  `Ssr` table and the stats queries aggregate in SQL.

### Security

- Admin session cookie sets the `Secure` flag in release builds (tunable via
  `SESSION_SECURE`).
- Admin `/auth/*` endpoints are always rate limited (burst + per-second
  replenish), independent of the `prod` feature.
- Image uploads (product and content) are rejected above 10 MiB.
- Postgres connections can use TLS via `PG_TLS` instead of always `NoTls`.

## [0.2] - 2026-08-22

### Added

- **Rich text editor with Slate.js** — `slate/` React app built with rspack:
  - structured JSON content stored in the `Content` table, rendered on the site
  - image blocks; uploading an image and cleaning up removed ones via content update
  - debounced autosave of the editor state
- **systemd units and ferron configs**

### Changed

- Migrated query client from Clorinde to Cornucopia 1.0.1 (clorinde is now cornucopia)
- Workspace projects to edition 2024
- Leptos 0.8.20 and dependency updates
- Auth uses `user_get_auth_by_username` for lookup

### Fixed

- `publish_at > now()` filtering for 404 on unpublished posts/products
- import order for cornucopia-generated code