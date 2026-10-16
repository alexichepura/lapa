//! `lapa-e2e` — Rust-native end-to-end harness (playwright-rs + testcontainers).
//!
//! Subcommands:
//!   install  download the Chromium build bundled with the Playwright driver
//!   serve    boot Postgres + the selected app(s), then sit idle
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
use apps::integration::IntegrationApp;
use apps::site::SiteApp;

#[derive(Parser)]
#[command(name = "lapa-e2e", about = "lapa end-to-end test harness")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Which suite to run. `integration` boots both apps and exercises cross-app
/// behaviour; the others boot a single app.
#[derive(Clone, Copy, Debug, ValueEnum)]
enum AppName {
    Site,
    Admin,
    Integration,
}

#[derive(Subcommand)]
enum Command {
    /// Install the Chromium browser required by the bundled Playwright driver.
    Install,
    /// Start Postgres + the app server(s) and keep them running.
    Serve {
        #[arg(long, value_enum, default_value_t = AppName::Site)]
        app: AppName,
    },
    /// Run the end-to-end suite against a freshly booted stack.
    Test {
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

const SITE: SiteApp = SiteApp;
const ADMIN: AdminApp = AdminApp;
const INTEGRATION: IntegrationApp = IntegrationApp;

/// Servers to boot for a suite.
fn servers_for(app: AppName) -> Vec<&'static dyn AppUnderTest> {
    match app {
        AppName::Site => vec![&SITE],
        AppName::Admin => vec![&ADMIN],
        AppName::Integration => vec![&SITE, &ADMIN],
    }
}

/// The suite (case collection + snapshot namespace) for a selection.
fn suite_for(app: AppName) -> &'static dyn AppUnderTest {
    match app {
        AppName::Site => &SITE,
        AppName::Admin => &ADMIN,
        AppName::Integration => &INTEGRATION,
    }
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
        Command::Serve { app } => run_serve(app).await?,
        Command::Test { app, filter } => run_test(app, filter.as_deref(), false).await?,
        Command::Update { app, filter } => run_test(app, filter.as_deref(), true).await?,
        Command::Shot { url, out } => {
            support::browser::shot(&url, &out).await?;
            println!("saved {}", out.display());
        }
    }

    Ok(())
}

async fn run_serve(app: AppName) -> Result<()> {
    let session = support::harness::Session::boot(&servers_for(app)).await?;
    session.run_until_interrupt().await
}

async fn run_test(app: AppName, filter: Option<&str>, update: bool) -> Result<()> {
    let servers = servers_for(app);
    let suite = suite_for(app);
    let session = support::harness::Session::boot(&servers).await?;
    let report = session.run(suite, filter, update).await?;
    let failed = report.failed();
    // Drop the session (stops servers, browser and containers) before exiting.
    drop(session);
    if failed {
        std::process::exit(1);
    }
    Ok(())
}
