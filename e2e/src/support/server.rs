//! Launches the Leptos site server (`cargo leptos serve`) against the ephemeral
//! database, captures its stdout/stderr, and waits for readiness.

use std::io::Write as _;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use tokio::io::AsyncRead;
use tokio::process::{Child, Command};
use tokio::time::{Instant, sleep};

/// Shared, append-only log sink for a child process. Lines are kept in memory
/// (for assertions) and mirrored to a file (for post-mortem debugging).
#[derive(Clone)]
pub struct ServerLog {
    inner: Arc<Mutex<LogInner>>,
}

struct LogInner {
    lines: Vec<String>,
    file: Option<std::fs::File>,
}

impl ServerLog {
    pub fn new(path: Option<&Path>) -> Self {
        let file = path.and_then(|p| {
            if let Some(parent) = p.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            std::fs::File::create(p).ok()
        });
        Self {
            inner: Arc::new(Mutex::new(LogInner {
                lines: Vec::new(),
                file,
            })),
        }
    }

    fn push(&self, line: &str) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(file) = guard.file.as_mut() {
            let _ = writeln!(file, "{line}");
        }
        guard.lines.push(line.to_string());
    }

    pub fn tail(&self, n: usize) -> String {
        let guard = self.inner.lock().unwrap();
        let start = guard.lines.len().saturating_sub(n);
        guard.lines[start..].join("\n")
    }

    /// Fatal server-side lines (panics, `ERROR` logs).
    pub fn errors(&self) -> Vec<String> {
        let guard = self.inner.lock().unwrap();
        guard
            .lines
            .iter()
            .filter(|line| is_fatal(line))
            .cloned()
            .collect()
    }

    /// Warnings worth surfacing but not failing on.
    pub fn warnings(&self) -> Vec<String> {
        let guard = self.inner.lock().unwrap();
        guard
            .lines
            .iter()
            .filter(|line| is_warning(line) && !is_fatal(line))
            .cloned()
            .collect()
    }
}

fn is_fatal(line: &str) -> bool {
    const FATAL: [&str; 4] = [
        "ERROR",
        "panicked at",
        "thread 'main'",
        "failed to create pool",
    ];
    FATAL.iter().any(|needle| line.contains(needle))
}

fn is_warning(line: &str) -> bool {
    line.contains("WARN") || line.contains("warning:")
}

pub struct ServerConfig<'a> {
    /// Directory passed to `cargo leptos` (the workspace root).
    pub server_dir: &'a Path,
    /// Leptos project to serve (`site` or `admin`).
    pub project: &'a str,
    pub database_url: &'a str,
    /// Where uploaded originals / converted derivatives are read from.
    pub image_upload_path: &'a str,
    pub image_convert_path: &'a str,
    pub timeout: Duration,
    pub log_path: Option<&'a Path>,
}

pub struct Server {
    child: Option<Child>,
    pgid: i32,
    port: u16,
    pub base_url: String,
    pub log: ServerLog,
}

impl Server {
    pub async fn start(cfg: ServerConfig<'_>) -> Result<Self> {
        let port = free_port()?;
        let addr = format!("127.0.0.1:{port}");
        let base_url = format!("http://{addr}");

        let mut cmd = Command::new("sh");
        // `--project <name>`: the workspace contains two Leptos apps (`site` and
        // `admin`), so cargo-leptos cannot pick one implicitly.
        cmd.arg("-c")
            .arg(format!("exec cargo leptos serve --project {}", cfg.project));
        cmd.current_dir(cfg.server_dir);
        // Lapa reads `pg.url` through the `config` crate's `__`-separated env;
        // `dotenv()` does not override an existing variable, so this wins over
        // the repo's `.env`.
        cmd.env("PG__URL", cfg.database_url);
        cmd.env("LEPTOS_SITE_ADDR", &addr);
        cmd.env("IMAGE_UPLOAD_PATH", cfg.image_upload_path);
        cmd.env("IMAGE_CONVERT_PATH", cfg.image_convert_path);
        cmd.env(
            "RUST_LOG",
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        );
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        #[cfg(unix)]
        cmd.process_group(0);

        let mut child = cmd
            .spawn()
            .context("failed to spawn `cargo leptos serve` (is cargo-leptos on PATH?)")?;
        let pgid = child.id().map(|id| id as i32).unwrap_or(0);

        let log = ServerLog::new(cfg.log_path);
        if let Some(stdout) = child.stdout.take() {
            spawn_reader(stdout, log.clone());
        }
        if let Some(stderr) = child.stderr.take() {
            spawn_reader(stderr, log.clone());
        }

        let deadline = Instant::now() + cfg.timeout;
        loop {
            if let Some(status) = child.try_wait().context("polling server process")? {
                bail!(
                    "server exited before becoming ready ({status})\n--- server log ---\n{}",
                    log.tail(60)
                );
            }
            if tokio::net::TcpStream::connect(&addr).await.is_ok() {
                break;
            }
            if Instant::now() >= deadline {
                bail!(
                    "timed out waiting for {addr} after {:?}\n--- server log ---\n{}",
                    cfg.timeout,
                    log.tail(60)
                );
            }
            sleep(Duration::from_millis(250)).await;
        }

        Ok(Self {
            child: Some(child),
            pgid,
            port,
            base_url,
            log,
        })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        // Kill the whole process group (sh → cargo-leptos).
        #[cfg(unix)]
        if self.pgid > 0 {
            unsafe {
                libc::kill(-self.pgid, libc::SIGKILL);
            }
        }
        // cargo-leptos runs the server binary in its own group, so also kill
        // whatever is listening on our ephemeral port.
        kill_listeners(self.port);
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
    }
}

/// Kill the process(es) listening on `port` (best effort, via `lsof`).
fn kill_listeners(port: u16) {
    let output = std::process::Command::new("lsof")
        .args(["-ti", &format!("tcp:{port}"), "-sTCP:LISTEN"])
        .output();
    let Ok(output) = output else {
        return;
    };
    for pid in String::from_utf8_lossy(&output.stdout).split_whitespace() {
        if let Ok(pid) = pid.parse::<i32>() {
            unsafe {
                libc::kill(pid, libc::SIGKILL);
            }
        }
    }
}

fn spawn_reader<R>(reader: R, log: ServerLog)
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let mut lines = BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            log.push(&line);
        }
    });
}

fn free_port() -> Result<u16> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}
