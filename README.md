# LAPA - Leptos Axum Prisma starter with Admin dashboard and SSR/SPA website

<img width="1512" alt="LAPA Admin Screenshot 2023-07-01 at 11 11 33" src="https://github.com/alexichepura/lapa/assets/5582266/21d19e52-8fe1-4497-93e3-49ef488b11df">


## CSS nesting and custom media
For now requires fork of cargo-leptos with relevant flags for lightningcss
https://github.com/leptos-rs/cargo-leptos/commit/da6c7aeec7b062335e6592ca379c175fb82d3c16
```sh
cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos --rev da6c7aeec7b062335e6592ca379c175fb82d3c16
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
    container-type: inline-size; /* required setup for container */
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
