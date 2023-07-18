<img width="128" alt="LAPA Logo" src="https://github.com/alexichepura/lapa/assets/5582266/d13a532e-dd04-48a5-af49-d5f8e9e75c6e">

# LAPA - Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website

![LAPA Admin screenshot 2023-07-10](https://github.com/alexichepura/lapa/assets/5582266/753ba3ff-1911-4d00-80cc-90888252f54f)

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
<https://github.com/leptos-rs/leptos> \
A cutting-edge, high-performance frontend framework SSR+SPA. Using reactive signals.

### Axum
<https://github.com/tokio-rs/axum> \
Backend framework built with Tokio, Tower, and Hyper. Focuses on ergonomics and modularity.

### Prisma
<https://www.prisma.io> \
<https://prisma.brendonovich.dev> \
<https://github.com/Brendonovich/prisma-client-rust> \
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
    - compression and precompression
    - open graph meta tags
- admin auth and session with 
    - axum_session <https://github.com/AscendingCreations/AxumSessions>
    - axum_session_auth <https://github.com/AscendingCreations/AxumSessionsAuth>
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
    - processing with <https://lightningcss.dev> (forked cargo-leptos for now)
        - nesting
        - custom media
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

## Notes on CSS nesting and custom media
For now requires fork of cargo-leptos with relevant flags for lightningcss
https://github.com/leptos-rs/cargo-leptos/commit/da6c7aeec7b062335e6592ca379c175fb82d3c16
```sh
cargo install --git https://github.com/alexichepura/cargo-leptos --branch lightningcss-parsing-flags --locked cargo-leptos
```

### CSS nesting
https://caniuse.com/css-nesting
```css
nav {
    /* nav styles */
	&>a {
        /* nav>a styles */
    }
}
```

### Custom media
```css
@custom-media --md-n-above (width >=768px);
@media (--md-n-above) {}
```

### Container query
https://caniuse.com/css-container-queries

```css
main form {
    container-type: inline-size;
}
@container (width >=480px) {
    form footer {
		grid-template-columns: auto auto;
	}
}
```

### Container query + custom media
NOTE Container query doesn't work with custom media yet
```css
@container (--sm-n-above) {} /* doesn't work */
@container (width >=480px) {} /* works */
```

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
