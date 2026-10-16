//! Boots the whole stack and runs an app's cases.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;

use crate::apps::{AppUnderTest, CaseCtx};

use super::browser::{self, BrowserSession};
use super::infra::Infra;
use super::seed;
use super::server::{Server, ServerConfig};

pub fn artifacts_dir() -> PathBuf {
    // Artifacts live in the workspace `target/` (`CARGO_TARGET_DIR` wins).
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("target")
        });
    let target = target.canonicalize().unwrap_or(target);
    target.join("e2e-artifacts")
}

#[derive(Default)]
pub struct TestReport {
    pub passed: Vec<String>,
    pub failed: Vec<(String, String)>,
}

impl TestReport {
    pub fn failed(&self) -> bool {
        !self.failed.is_empty()
    }
}

pub struct Session {
    // Drop order matters: stop the server and browser before the containers.
    server: Server,
    browser: BrowserSession,
    _infra: Infra,
    pub base_url: String,
}

impl Session {
    pub async fn boot<A: AppUnderTest>(app: &A) -> Result<Self> {
        let infra = Infra::start().await?;
        println!("infra ready (database)");

        let artifacts = artifacts_dir();
        std::fs::create_dir_all(&artifacts).ok();
        let log_path = artifacts.join("server.log");

        let server_dir = app.server_dir();
        let server = Server::start(ServerConfig {
            server_dir: &server_dir,
            database_url: &infra.database_url,
            timeout: Duration::from_secs(900),
            log_path: Some(&log_path),
        })
        .await?;
        println!("server ready at {}", server.base_url);

        // The server only connects lazily; apply the schema + seed now so the
        // first request already sees the fixture rows.
        seed::apply(&infra.database_url).await?;
        println!("seeded deterministic fixtures");

        let browser = browser::launch().await?;
        let base_url = server.base_url.clone();

        Ok(Self {
            _infra: infra,
            server,
            browser,
            base_url,
        })
    }

    pub async fn run<A: AppUnderTest>(
        &self,
        app: &A,
        filter: Option<&str>,
        update: bool,
    ) -> Result<TestReport> {
        let artifacts = artifacts_dir();
        let mut report = TestReport::default();

        println!("running `{}` cases…", app.name());
        for case in app.cases() {
            if let Some(filter) = filter
                && !case.name.contains(filter)
            {
                continue;
            }
            let ctx = CaseCtx {
                app: app.name(),
                base_url: self.base_url.clone(),
                page: self.browser.page.clone(),
                artifacts_dir: artifacts.clone(),
                server_log: self.server.log.clone(),
                update,
            };
            match (case.run)(&ctx).await {
                Ok(()) => {
                    println!("  ok    {}", case.name);
                    report.passed.push(case.name.to_string());
                }
                Err(err) => {
                    println!("  FAIL  {}: {err:#}", case.name);
                    report
                        .failed
                        .push((case.name.to_string(), format!("{err:#}")));
                }
            }
        }

        // Give the log readers a moment to drain, then surface fatal server logs.
        tokio::time::sleep(Duration::from_millis(300)).await;
        let fatal = self.server.log.errors();
        if !fatal.is_empty() {
            let detail = fatal.join("\n  ");
            println!("  FAIL  server-log: {detail}");
            report.failed.push(("server-log".to_string(), detail));
        }
        let warnings = self.server.log.warnings();
        if !warnings.is_empty() {
            println!(
                "  note  {} server warning(s), see {}",
                warnings.len(),
                artifacts.join("server.log").display()
            );
        }

        report.print_summary();
        Ok(report)
    }

    /// Keep the stack alive for manual/agent browsing until Ctrl-C.
    pub async fn run_until_interrupt(&self) -> Result<()> {
        println!("artifacts: {}", artifacts_dir().display());
        println!("press Ctrl-C to stop");
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}

impl TestReport {
    pub fn print_summary(&self) {
        println!(
            "\n{} passed, {} failed",
            self.passed.len(),
            self.failed.len()
        );
        for (name, detail) in &self.failed {
            println!("  - {name}: {detail}");
        }
    }
}
