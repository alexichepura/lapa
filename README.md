<img width="128" alt="Lapa Logo" src="https://github.com/alexichepura/lapa/assets/5582266/d13a532e-dd04-48a5-af49-d5f8e9e75c6e">

# Lapa - Leptos Axum starter with Admin dashboard and SSR/SPA website

## Status
This project is under active development.\
Recently migrated from Prisma ORM to SQL first approach with [Cornucopia](https://github.com/cornucopia-rs/cornucopia).\
Also added integration with Slate.js for rich text editor because we don't have a good one in Rust yet.\

---

Intro: <https://youtu.be/6eMWAI1D-XA> \
Demo site: <https://lapa.chepura.space>

---
Screenshots below are outdated and to be updated.

<img width="360" alt="Site Home" src="https://github.com/alexichepura/lapa/assets/5582266/66326ce4-c61c-4fcc-a9f3-1f0548bb8c60">
<img width="360" alt="Site Home" src="https://github.com/alexichepura/lapa/assets/5582266/4a0f0d99-fd95-4abe-84bb-30e43c9aeeaa">

<img width="360" alt="Admin Dashboard" src="https://github.com/alexichepura/lapa/assets/5582266/cfb71304-9fdf-45c1-bd94-85ec90f07a0f">
<img width="360" alt="Admin Posts" src="https://github.com/alexichepura/lapa/assets/5582266/7f0219cf-f231-4559-bffe-faec6e7b9285">
<img width="360" alt="Admin Post" src="https://github.com/alexichepura/lapa/assets/5582266/d7ce5c3e-3686-4d41-9da3-e898ef7d2cad">
<img width="360" alt="Admin Post" src="https://github.com/alexichepura/lapa/assets/5582266/3f08bf40-9c80-4b6a-8a5b-b9b2b8732066">
<img width="360" alt="Admin Settings" src="https://github.com/alexichepura/lapa/assets/5582266/b2913992-bfb2-4454-83f3-f526a73fbb49">
<img width="90" alt="Admin Mobile" src="https://github.com/alexichepura/lapa/assets/5582266/96f10565-19da-4b8e-80c0-6125bb5a97ac">

## Motivation

I want to have practical full-stack setup to build websites and services. \
Utilising type safety and performance of Rust opens the door for new era of web dev, that is taking off. \
Ecosystem and standardized approach is helpful to develop scalable and future-proof apps. \
Some benefits:

- strict types
- enforced error and value management (Result and Option)
- predictable performance (no garbage collector)
- native performance
- single bundler (cargo)
- straight path to WebAssembly

### Leptos

<https://leptos.dev> \
[leptos-rs/leptos](https://github.com/leptos-rs/leptos) \
A cutting-edge, high-performance frontend framework SSR+SPA. Using reactive signals.

### Axum

[tokio-rs/axum](https://github.com/tokio-rs/axum) \
Backend framework built with Tokio, Tower, and Hyper. Focuses on ergonomics and modularity.

## Features

- project
  - SEO site
  - admin dashboard
  - CLI with clap: migrate, settings-init, user-add, user-disable, user-enable, user-set-role
  - ops scripts: build, upload, run (site, admin, cli)
- site
  - SSR + SPA hydrated
  - open graph meta tags
- prod features
  - ratelimit with [benwis/tower-governor](https://github.com/benwis/tower-governor)
  - compression with tower-http/compression
  - precompression with [ryanfowler/precompress](https://github.com/ryanfowler/precompress) see ./ops scripts
- admin auth and session with
  - axum_session [AscendingCreations/AxumSessions](https://github.com/AscendingCreations/AxumSessions)
  - axum_session_auth [AscendingCreations/AxumSessionsAuth](https://github.com/AscendingCreations/AxumSessionsAuth)
- rich text content editor integration with [ianstormtaylor/slatejs](https://github.com/ianstormtaylor/slate) 
- post
  - CRUDL
  - publish_at
- product
  - CRUDL
  - publish_at
  - product images
    - preview and upload
    - resize and convert on backend
    - order
    - is_hero flag
    - delete and alt update in "dialog"
- settings
  - robots.txt, site_url
  - home_text
- css
  - based on <https://open-props.style>
  - dark and light themes
  - mobile first
  - CSS modules with [stylance](https://github.com/basro/stylance-rs), bundled by
    Lightning CSS (`style-bundle`) in cargo-leptos, see notes on css below
- components
  - forms, inputs and response messages
  - input datetime-local usage with chrono library
  - RoutingProgress
  - Favicons

## Run

### Generate db client
https://cornucopia-rs.github.io/cornucopia/introduction/installation.html \
Note the need of docker or podman.
```sh
cargo install cornucopia
cornucopia schema ./schema.sql
```

### Build rich text editor

```sh
cd slate && npm run build && cd -
```

### Local DB

Simplified with postgres user.
```sh
psql postgres
# create database lapa;
# .env
PG__URL="postgresql://postgres:postgres@localhost:5432/lapa"
```

### Environment

Runtime environment variables (all optional):

- `PG__URL` — Postgres connection string (see above).
- `PG_TLS` — set to `1`/`true` to connect to Postgres over TLS (rustls with
  Mozilla roots). Leave unset for a local socket or trusted-network plaintext
  connection.
- `SESSION_SECURE` — force the admin session cookie `Secure` flag (`1`/`0`).
  Defaults to on for release builds, off for debug.
- `AUTH_RATELIMIT_BURST` / `AUTH_RATELIMIT_PERIOD_SECS` — admin `/auth/*` rate
  limit (defaults: burst 10, then one token per second per client).
- `IMAGE_UPLOAD_PATH` / `IMAGE_CONVERT_PATH` — image storage roots.

### Migrations

The schema is managed with [refinery](https://docs.rs/refinery) migrations in
`db/migrations/`, embedded in the `cli` binary. `db/migrations/V1__init.sql`
mirrors `schema.sql` (kept for Cornucopia codegen, see above).

```sh
cargo cli migrate
```

Add a migration by creating the next file (e.g. `db/migrations/V2__add_thing.sql`)
and running `cargo cli migrate` again.

### Init

```sh
RUST_LOG="debug" cargo cli settings-init
RUST_LOG="debug" cargo cli user-add <username> <password>
```

### User management

Users carry a nullable `deactivated_at` timestamp (added in migration
`V2__user_deactivated_at.sql`): `NULL` means active, a timestamp records when the
account was disabled. A disabled account cannot log in, and any session it
already holds is rejected on the next request (the `axum-login` backend returns
no user for a disabled account), so disabling takes effect immediately without
deleting the account.

```sh
cargo cli user-disable <username>   # revoke access (also purges their sessions)
cargo cli user-enable <username>    # restore access
```

Both commands exit non-zero when no such user exists.

Users also have a single `role` (migration `V4__user_role.sql`):

| role     | can                                                        |
| -------- | ---------------------------------------------------------- |
| `admin`  | everything                                                 |
| `editor` | product/post/category/content CRUD, but not site settings  |
| `viewer` | read-only (dashboard and lists)                            |

The same permissions are enforced by the server functions; the admin UI hides
nav/buttons and renders a "Forbidden" page for gated routes. Set a role with:

```sh
cargo cli user-set-role <username> admin|editor|viewer
```

### Dev

```sh
cargo install stylance-cli   # one-time
stylance --watch .           # site CSS modules (run in a second terminal)
stylance --watch admin       # admin CSS modules (run in a second terminal)
RUST_LOG="info,admin=debug" cargo leptos watch -p admin
RUST_LOG="info,site=debug" cargo leptos watch -p site
```
Tip to enable tower_http debug
```sh
RUST_LOG="info,tower_http=debug,admin=debug" cargo leptos watch -p admin
RUST_LOG="info,tower_http=debug,site=debug" cargo leptos watch -p site
```

### Prod

See relevant tutorial and demo project.
<https://github.com/alexichepura/leptos_axum_prisma_sozu>
<https://www.youtube.com/watch?v=KLg8Hcd3K_U>

```sh
cargo leptos build --release
```

Production with compress and ratelimit

```sh
cargo leptos build --release --features="compression,ratelimit"
# or
cargo leptos build --release --features="prod"
```

### Ops

./ops folder contains example scripts to prepare production build and deploy it on server.
Check .env.example
Requires <https://github.com/ryanfowler/precompress>

```sh
./ops/site-deploy.sh && ./ops/site-run.sh # build, deploy and run site
./ops/admin-deploy.sh && ./ops/admin-run.sh # build, deploy and run admin
```

```sh
./ops/cli-deploy.sh # upload cli to server
```

### E2E tests

Rust-native end-to-end harness (`playwright-rs` + `testcontainers`) that boots
an ephemeral Postgres, applies `schema.sql`, seeds fixtures, starts the site and
drives Chromium. See `e2e/README.md`.

```sh
cargo install cargo-leptos
cargo run -p lapa-e2e -- install          # download Chromium (one-time)
cargo run -p lapa-e2e -- test             # public site suite
cargo run -p lapa-e2e -- test --app admin  # admin suite (auth + CRUD)
cargo run -p lapa-e2e -- test --app integration  # cross-app: admin write -> site read
cargo run -p lapa-e2e -- test --filter home
cargo run -p lapa-e2e -- update           # regenerate screenshot baselines
cargo run -p lapa-e2e -- serve            # keep the stack up for manual checks
cargo run -p lapa-e2e -- test --features prod  # build with compression + ratelimit
```

The suites also run in CI (`.github/workflows/e2e.yml`); CI sets
`LAPA_E2E_SKIP_SNAPSHOTS=1` since only macOS baselines are committed.

## Notes on CSS

Styling is plain CSS organised as **CSS modules** with
[`stylance`](https://github.com/basro/stylance-rs) plus Lightning CSS for
bundling (cargo-leptos `style-bundle = true`).

- Component-scoped rules live next to the components as `*.module.css`
  (e.g. `src/product_list.module.css`). Add the
  `stylance::import_crate_style!(style, "src/product_list.module.css");` import
  and reference `style::ProductList`; the macro resolves the same deterministic
  hash the CLI writes into the bundle.
- Shared/global element rules (e.g. `body`, `dialog`, `@container`) live in
  `style/main.css`, which also `@import`s the vendor stylesheets
  (`open-props`, `normalize`, `buttons`) and the generated `style/stylance.css`.
- Regenerate the bundle with `stylance .` (site) / `stylance admin` (admin), or
  keep `stylance --watch <crate>` running while editing. The generated
  `style/stylance.css` is committed so a plain `cargo leptos build` works
  without the CLI; `ops/*-deploy.sh` and the e2e harness refresh it for you.

Native CSS nesting, `@media`, `@container` and `@layer` are all supported;
Lightning CSS lowers them for the configured `browserquery`.

Earlier experiments: Sass PR <https://github.com/alexichepura/lapa/pull/24>,
Lightning CSS bundle with CLI proof of concept
<https://github.com/alexichepura/lapa/pull/23>.

## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
