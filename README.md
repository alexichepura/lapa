<img width="128" alt="LAPA Logo" src="https://github.com/alexichepura/lapa/assets/5582266/d13a532e-dd04-48a5-af49-d5f8e9e75c6e">

# LAPA - Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website

<img width="1512" alt="LAPA Admin Screenshot 2023-07-01" src="https://github.com/alexichepura/lapa/assets/5582266/21d19e52-8fe1-4497-93e3-49ef488b11df">

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

## Includes
- monorepo with 2 main packages for SEO site and admin dashboard
- prisma schema for db (user, session, post, image, settings)
- admin auth and session with 
    - axum_session <https://github.com/AscendingCreations/AxumSessions>
    - axum_session_auth <https://github.com/AscendingCreations/AxumSessionsAuth>
    - custom adapter for DatabasePool to use prisma
- image preview and upload
- images resize and convert on backend
- css based on <https://open-props.style>
- css processing with <https://lightningcss.dev> (forked cargo-leptos for now)
    - nesting
    - custom media
- compression and precompression
- stiched together forms, inputs and response messages
- input datetime-local usage with chrono library
- robots.txt from database
- RoutingProgress
- Favicons

## Run 
Requires prisma client to be generated
```sh
cargo prisma db push # generate client and push schema to db
# or
cargo prisma generate # only generate client
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
```

## Notes on CSS nesting and custom media
For now requires fork of cargo-leptos with relevant flags for lightningcss
https://github.com/leptos-rs/cargo-leptos/commit/da6c7aeec7b062335e6592ca379c175fb82d3c16
```sh
cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos --rev 7d141f13eca0a401d030f2e038192eb71ecfebe8
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


## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
