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
cargo run -p lapa-e2e -- test                 # full case suite
cargo run -p lapa-e2e -- test --filter home    # only cases whose name contains "home"
cargo run -p lapa-e2e -- update                # regenerate screenshot baselines
cargo run -p lapa-e2e -- serve                 # infra + server, waits for Ctrl-C
cargo run -p lapa-e2e -- shot http://127.0.0.1:<port>/ /tmp/page.png
```

`serve` prints the base URL and keeps the stack up, which is handy for opening a
page manually or taking a screenshot with `shot`.

The first run compiles the site's SSR + wasm bundles, so give it a few minutes.

## Layout

- `e2e/snapshots/site/<os>/*.png` — baseline screenshots, committed.
- `target/e2e-artifacts/` — run artifacts: `server.log` (server stdout/stderr)
  and `*.actual.png` on a screenshot mismatch. Not committed.
- `e2e/src/apps/mod.rs` — the `AppUnderTest` trait and `CaseCtx`.
- `e2e/src/apps/site.rs` — the public-site cases.
- `e2e/src/support/` — containers, server process, browser, seed and snapshots.

## Adding a case

1. Write `fn case_x<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a>` in `apps/site.rs`.
2. Register it in `SiteApp::cases()`.
3. For a visual case, call `ctx.snapshot("name")`; the first `update` run
   creates the baseline, later runs compare against it with a pixel diff.
