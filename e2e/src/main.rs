//! `lapa-e2e` — Rust-native end-to-end harness (playwright-rs + testcontainers).
//!
//! Subcommands:
//!   install  download the Chromium build bundled with the Playwright driver
//!   serve    boot Postgres + the Leptos site, then sit idle
//!   test     boot the whole stack and run the smoke suite
//!   update   like `test` but rewrites screenshot baselines
//!   shot     screenshot an arbitrary URL with a throwaway browser

mod apps;
mod support;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use apps::site::SiteApp;

#[derive(Parser)]
#[command(name = "lapa-e2e", about = "lapa end-to-end test harness")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install the Chromium browser required by the bundled Playwright driver.
    Install,
    /// Start Postgres + the site server and keep them running.
    Serve,
    /// Run the end-to-end suite against a freshly booted stack.
    Test {
        /// Only run cases whose name contains this substring.
        #[arg(long)]
        filter: Option<String>,
    },
    /// Regenerate screenshot baselines.
    Update {
        #[arg(long)]
        filter: Option<String>,
    },
    /// Screenshot a URL with a throwaway browser (against an already running server).
    Shot { url: String, out: PathBuf },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    match Cli::parse().command {
        Command::Install => {
            println!("installing chromium (this downloads a browser build)…");
            playwright_rs::install_browsers(Some(&["chromium"])).await?;
            println!("done");
        }
        Command::Serve => {
            let session = support::harness::Session::boot(&SiteApp).await?;
            session.run_until_interrupt().await?;
        }
        Command::Test { filter } => {
            let session = support::harness::Session::boot(&SiteApp).await?;
            let report = session.run(&SiteApp, filter.as_deref(), false).await?;
            let failed = report.failed();
            // Drop the session (stops server, browser and containers) before exiting.
            drop(session);
            if failed {
                std::process::exit(1);
            }
        }
        Command::Update { filter } => {
            let session = support::harness::Session::boot(&SiteApp).await?;
            let report = session.run(&SiteApp, filter.as_deref(), true).await?;
            let failed = report.failed();
            drop(session);
            if failed {
                std::process::exit(1);
            }
        }
        Command::Shot { url, out } => {
            support::browser::shot(&url, &out).await?;
            println!("saved {}", out.display());
        }
    }

    Ok(())
}
