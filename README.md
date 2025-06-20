<img width="128" alt="Lapa Logo" src="https://github.com/alexichepura/lapa/assets/5582266/d13a532e-dd04-48a5-af49-d5f8e9e75c6e">

# Lapa - Leptos Axum starter with Admin dashboard and SSR/SPA website

## Status
This project is under active development.\
Recently migrated from Prisma ORM to SQL first approach with [Clorinde](https://github.com/halcyonnouveau/clorinde).\
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
  - CLI with clap: settings-init, user-add
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
  - sass, @custom-media, @container, see notes on css below
- components
  - forms, inputs and response messages
  - input datetime-local usage with chrono library
  - RoutingProgress
  - Favicons

## Run

### Generate db client

```sh
clorinde schema ./schema.sql
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
psql -h 127.0.0.1 -U postgres -p 5432 -d lapa < "schema.sql"
# .env
PG__URL="postgresql://postgres:postgres@localhost:5432/lapa"
```

### Init

```sh
RUST_LOG="debug" cargo cli settings-init
RUST_LOG="debug" cargo cli user-add
```

### Dev

```sh
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

## Notes on CSS

Modern CSS is quite cool. Nesting, custom media, container queries. All that was used here before, but required cargo-leptos fork. As well another cli step to bundle everything into one css. For now returning to SASS.
Considering return back to CSS if/when cargo-leptos will support lightningcss config and bundling.

Sass PR <https://github.com/alexichepura/lapa/pull/24>.
Ligntningcss bundle with cli proof of concept <https://github.com/alexichepura/lapa/pull/23>.

## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
