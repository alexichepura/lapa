# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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