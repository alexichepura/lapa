//! `lapa-e2e` — Rust-native end-to-end harness (playwright-rs + testcontainers).
//!
//! Subcommands:
//!   install  download the Chromium build bundled with the Playwright driver
//!   serve    boot Postgres + a Leptos app, then sit idle
//!   test     boot the whole stack and run the smoke suite
//!   update   like `test` but rewrites screenshot baselines
//!   shot     screenshot an arbitrary URL with a throwaway browser

mod apps;
mod support;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use apps::AppUnderTest;
use apps::admin::AdminApp;
use apps::site::SiteApp;

#[derive(Parser)]
#[command(name = "lapa-e2e", about = "lapa end-to-end test harness")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum AppName {
    Site,
    Admin,
}

#[derive(Subcommand)]
enum Command {
    /// Install the Chromium browser required by the bundled Playwright driver.
    Install,
    /// Start Postgres + the app server and keep them running.
    Serve {
        /// Which Leptos app to serve.
        #[arg(long, value_enum, default_value_t = AppName::Site)]
        app: AppName,
    },
    /// Run the end-to-end suite against a freshly booted stack.
    Test {
        /// Which Leptos app to exercise.
        #[arg(long, value_enum, default_value_t = AppName::Site)]
        app: AppName,
        /// Only run cases whose name contains this substring.
        #[arg(long)]
        filter: Option<String>,
    },
    /// Regenerate screenshot baselines.
    Update {
        #[arg(long, value_enum, default_value_t = AppName::Site)]
        app: AppName,
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
        Command::Serve { app } => match app {
            AppName::Site => run_serve(&SiteApp).await?,
            AppName::Admin => run_serve(&AdminApp).await?,
        },
        Command::Test { app, filter } => match app {
            AppName::Site => run_test(&SiteApp, filter.as_deref(), false).await?,
            AppName::Admin => run_test(&AdminApp, filter.as_deref(), false).await?,
        },
        Command::Update { app, filter } => match app {
            AppName::Site => run_test(&SiteApp, filter.as_deref(), true).await?,
            AppName::Admin => run_test(&AdminApp, filter.as_deref(), true).await?,
        },
        Command::Shot { url, out } => {
            support::browser::shot(&url, &out).await?;
            println!("saved {}", out.display());
        }
    }

    Ok(())
}

async fn run_serve<A: AppUnderTest>(app: &A) -> Result<()> {
    let session = support::harness::Session::boot(app).await?;
    session.run_until_interrupt().await
}

async fn run_test<A: AppUnderTest>(app: &A, filter: Option<&str>, update: bool) -> Result<()> {
    let session = support::harness::Session::boot(app).await?;
    let report = session.run(app, filter, update).await?;
    let failed = report.failed();
    // Drop the session (stops server, browser and containers) before exiting.
    drop(session);
    if failed {
        std::process::exit(1);
    }
    Ok(())
}
