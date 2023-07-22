<img width="128" alt="LAPA Logo" src="https://github.com/alexichepura/lapa/assets/5582266/d13a532e-dd04-48a5-af49-d5f8e9e75c6e">

# LAPA - Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website

<img width="360" alt="Site Home" src="https://github.com/alexichepura/lapa/assets/5582266/66326ce4-c61c-4fcc-a9f3-1f0548bb8c60">
<img width="360" alt="Site Home" src="https://github.com/alexichepura/lapa/assets/5582266/4a0f0d99-fd95-4abe-84bb-30e43c9aeeaa">

<img width="360" alt="Admin Dashboard" src="https://github.com/alexichepura/lapa/assets/5582266/cfb71304-9fdf-45c1-bd94-85ec90f07a0f">
<img width="360" alt="Admin Posts" src="https://github.com/alexichepura/lapa/assets/5582266/7f0219cf-f231-4559-bffe-faec6e7b9285">
<img width="360" alt="Admin Post" src="https://github.com/alexichepura/lapa/assets/5582266/d7ce5c3e-3686-4d41-9da3-e898ef7d2cad">
<img width="360" alt="Admin Post" src="https://github.com/alexichepura/lapa/assets/5582266/3f08bf40-9c80-4b6a-8a5b-b9b2b8732066">
<img width="360" alt="Admin Settings" src="https://github.com/alexichepura/lapa/assets/5582266/b2913992-bfb2-4454-83f3-f526a73fbb49">
<img width="90" alt="Admin Mobile" src="https://github.com/alexichepura/lapa/assets/5582266/96f10565-19da-4b8e-80c0-6125bb5a97ac">

DEMO site <https://lapa.chepura.space>

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

## 3 pillars
### Leptos
<https://leptos.dev> \
[leptos-rs/leptos](https://github.com/leptos-rs/leptos) \
A cutting-edge, high-performance frontend framework SSR+SPA. Using reactive signals.

### Axum
[tokio-rs/axum](https://github.com/tokio-rs/axum) \
Backend framework built with Tokio, Tower, and Hyper. Focuses on ergonomics and modularity.

### Prisma
<https://www.prisma.io> \
<https://prisma.brendonovich.dev> \
[Brendonovich/prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust) \
Type-safe database access.

## Features
- project
    - SEO site
    - admin dashboard
    - CLI with clap: settings-init, user-add, migrate
    - prisma schema: user, session, post, image, settings
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
    - custom prisma DatabasePool
- post 
    - admin CRUDL
    - published_at
- images
    - preview and upload
    - resize and convert on backend
    - order in gallery
    - is_hero flag
    - delete and alt update in "dialog"
- settings
    - robots.txt, site_url
    - images sizes
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
### Generate prisma client
```sh
cargo prisma db push # generate client and push schema to db
# or
cargo prisma generate # only generate client
```
### Init 
```sh
cargo lapa settings-init
cargo lapa user-add
```

### Dev
```sh
cargo leptos watch -p lapa_admin
cargo leptos watch -p lapa_site
```

### Prod
See relevant tutorial and demo project.
<https://github.com/alexichepura/leptos_axum_prisma_sozu>
<https://www.youtube.com/watch?v=KLg8Hcd3K_U>
```sh
cargo leptos build --release
cargo leptos build --release --features="prod"
cargo leptos build --release --features="prod"
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
./ops/prisma-upload.sh # upload prisma folder with migrations to server
./ops/cli-deploy.sh # upload cli to server
```

## Notes on CSS
Modern CSS is quite cool. Nesting, custom media, container queries. All that was used here before, but required cargo-leptos fork. As well another cli step to bundle everything into one css. For now returning to SASS. 
Considering return back to CSS if/when cargo-leptos will support lightningcss config and bundling.

Sass PR <https://github.com/alexichepura/lapa/pull/24>.
Ligntningcss bundle with cli proof of concept <https://github.com/alexichepura/lapa/pull/23>.


## Notes on prisma
How initial migration created
<https://www.prisma.io/docs/guides/migrate/developing-with-prisma-migrate/add-prisma-migrate-to-a-project>
```sh
mkdir -p prisma/migrations/0_init
cargo prisma migrate diff --from-empty --to-schema-datamodel prisma/schema.prisma --script > prisma/migrations/0_init/migration.sql
cargo prisma migrate resolve --applied 0_init
```

## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
