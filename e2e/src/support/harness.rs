//! Boots the whole stack and runs an app's cases.

use std::collections::HashMap;
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
    // Drop order matters: stop the servers and browser before the containers.
    servers: Vec<(&'static str, Server)>,
    browser: BrowserSession,
    _infra: Infra,
    base_urls: HashMap<&'static str, String>,
}

impl Session {
    /// Boot every app in `apps` against one shared database + browser.
    pub async fn boot(apps: &[&dyn AppUnderTest]) -> Result<Self> {
        let infra = Infra::start().await?;
        println!("infra ready (database)");

        let artifacts = artifacts_dir();
        std::fs::create_dir_all(&artifacts).ok();

        // Serve uploads/derivatives out of the artifacts dir instead of the
        // repo's `image_upload/`/`image_convert/`, so runs do not litter the
        // working tree.
        let image_upload = artifacts.join("image_upload");
        let image_convert = artifacts.join("image_convert");
        for sub in ["content", "product"] {
            std::fs::create_dir_all(image_convert.join(sub)).ok();
        }
        let image_upload_path = image_upload.to_string_lossy().to_string();
        let image_convert_path = image_convert.to_string_lossy().to_string();

        let mut servers = Vec::new();
        let mut base_urls = HashMap::new();
        for app in apps {
            let name = app.name();
            let log_path = artifacts.join(format!("server-{name}.log"));
            let server_dir = app.server_dir();
            let server = Server::start(ServerConfig {
                server_dir: &server_dir,
                project: name,
                database_url: &infra.database_url,
                image_upload_path: &image_upload_path,
                image_convert_path: &image_convert_path,
                timeout: Duration::from_secs(900),
                log_path: Some(&log_path),
            })
            .await?;
            println!("server `{name}` ready at {}", server.base_url);
            base_urls.insert(name, server.base_url.clone());
            servers.push((name, server));
        }

        // The servers connect lazily; apply the schema + seed now so the first
        // request already sees the fixture rows.
        seed::apply(&infra.database_url, &image_upload).await?;
        println!("seeded deterministic fixtures");

        let browser = browser::launch().await?;

        Ok(Self {
            servers,
            browser,
            _infra: infra,
            base_urls,
        })
    }

    pub async fn run(
        &self,
        suite: &dyn AppUnderTest,
        filter: Option<&str>,
        update: bool,
    ) -> Result<TestReport> {
        let artifacts = artifacts_dir();
        let mut report = TestReport::default();

        // The suite's own app is the default target for `ctx.goto`; a cross-app
        // suite (not a server itself) defaults to the first booted server.
        let primary = if self.base_urls.contains_key(suite.name()) {
            suite.name()
        } else {
            self.base_urls.keys().copied().next().unwrap_or("site")
        };

        println!("running `{}` cases…", suite.name());
        for case in suite.cases() {
            if let Some(filter) = filter
                && !case.name.contains(filter)
            {
                continue;
            }
            let ctx = CaseCtx {
                app: suite.name(),
                primary,
                base_urls: self.base_urls.clone(),
                page: self.browser.page.clone(),
                artifacts_dir: artifacts.clone(),
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
        for (name, server) in &self.servers {
            let fatal = server.log.errors();
            if !fatal.is_empty() {
                let detail = fatal.join("\n  ");
                println!("  FAIL  server-log[{name}]: {detail}");
                report.failed.push((format!("server-log[{name}]"), detail));
            }
            let warnings = server.log.warnings();
            if !warnings.is_empty() {
                println!(
                    "  note  {name}: {} server warning(s), see {}",
                    warnings.len(),
                    artifacts.join(format!("server-{name}.log")).display()
                );
            }
        }

        report.print_summary();
        Ok(report)
    }

    /// Keep the stack alive for manual/agent browsing until Ctrl-C.
    pub async fn run_until_interrupt(&self) -> Result<()> {
        println!("artifacts: {}", artifacts_dir().display());
        for (name, server) in &self.servers {
            println!("  {name}: {}", server.base_url);
        }
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
