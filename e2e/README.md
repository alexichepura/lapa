# lapa-e2e

Rust-native end-to-end harness for the Lapa site, built on
[`playwright-rs`](https://crates.io/crates/playwright-rs) +
[`testcontainers`](https://crates.io/crates/testcontainers). It boots an
ephemeral Postgres, applies `schema.sql`, seeds deterministic fixtures, starts
the Leptos site (`cargo leptos serve`) and drives Chromium against it.

## Prerequisites (one-time)

```sh
docker info                        # Docker must be running
cargo install cargo-leptos
cargo run -p lapa-e2e -- install   # download Chromium for the playwright-rs driver
```

## Commands

```sh
cargo run -p lapa-e2e -- test                  # site suite
cargo run -p lapa-e2e -- test --app admin       # admin suite (login + sections)
cargo run -p lapa-e2e -- test --filter home      # only cases whose name contains "home"
cargo run -p lapa-e2e -- update                 # regenerate site screenshot baselines
cargo run -p lapa-e2e -- update --app admin     # regenerate admin baselines
cargo run -p lapa-e2e -- serve                  # infra + site server, waits for Ctrl-C
cargo run -p lapa-e2e -- serve --app admin
cargo run -p lapa-e2e -- shot http://127.0.0.1:<port>/ /tmp/page.png
```

`serve` prints the base URL and keeps the stack up, which is handy for opening a
page manually or taking a screenshot with `shot`.

The first run compiles the app's SSR + wasm bundles, so give it a few minutes.

## Coverage

`site` (public): home/product/post render, `publish_at` gating + 404s, the
image pipeline (product hero + in-content image resized on demand), SEO meta
tags, client-side navigation, `robots.txt`, unmatched-route 404 and a
console-error sweep.

`admin`: unauthenticated redirect to the login form, the seeded-user login flow
and a console-error sweep across the dashboard/product/post/settings sections.

## Layout

- `e2e/snapshots/<app>/<os>/*.png` — baseline screenshots, committed.
- `e2e/fixtures/tiny.png` — 1x1 PNG seeded as an uploaded original so the CDN
  resize path can be exercised.
- `target/e2e-artifacts/` — run artifacts: `server.log` (server stdout/stderr),
  `*.actual.png` on a screenshot mismatch, and the `image_upload/` originals /
  `image_convert/` derivatives the server reads and writes. Not committed.
- `e2e/src/apps/mod.rs` — the `AppUnderTest` trait and `CaseCtx`.
- `e2e/src/apps/site.rs`, `e2e/src/apps/admin.rs` — the per-app cases.
- `e2e/src/support/` — containers, server process, browser, seed and snapshots.

## Adding a case

1. Write `fn case_x<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a>` in the app module.
2. Register it in `<App>::cases()`.
3. For a visual case, call `ctx.snapshot("name")`; the first `update` run
   creates the baseline, later runs compare against it with a pixel diff.

## Adding a new frontend

Implement `AppUnderTest` (`name`, `server_dir`, `cases`) in
`src/apps/<client>.rs`, add it to the `AppName` enum in `main.rs` and wire the
match arms. Infrastructure, the browser, seeds and snapshot handling are shared.

