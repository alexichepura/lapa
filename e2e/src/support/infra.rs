//! Ephemeral infrastructure for the e2e stack: a Postgres container on a random
//! host port. Lapa is a plain `tokio-postgres`/cornucopia app (no extensions),
//! so the stock `postgres` image is enough.

use std::borrow::Cow;

use anyhow::{Context, Result, bail};
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, Image};

pub const DB_USER: &str = "postgres";
pub const DB_PASSWORD: &str = "postgres";
pub const DB_NAME: &str = "lapa";

const POSTGRES_IMAGE: &str = "postgres";
const POSTGRES_TAG: &str = "17-alpine";
const POSTGRES_PORTS: &[ContainerPort] = &[ContainerPort::Tcp(5432)];

#[derive(Debug, Clone)]
pub struct Postgres {
    user: String,
    password: String,
    db: String,
}

impl Postgres {
    pub fn new(
        user: impl Into<String>,
        password: impl Into<String>,
        db: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            password: password.into(),
            db: db.into(),
        }
    }
}

impl Default for Postgres {
    fn default() -> Self {
        Self::new(DB_USER, DB_PASSWORD, DB_NAME)
    }
}

impl Image for Postgres {
    fn name(&self) -> &str {
        POSTGRES_IMAGE
    }

    fn tag(&self) -> &str {
        POSTGRES_TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        // The entrypoint prints this twice (temporary initdb server, then the
        // real one); `Infra::start` still polls the port so the first line is a
        // fine "container has booted" signal.
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        [
            ("POSTGRES_USER", self.user.clone()),
            ("POSTGRES_PASSWORD", self.password.clone()),
            ("POSTGRES_DB", self.db.clone()),
        ]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        POSTGRES_PORTS
    }
}

/// Running infrastructure. Dropping this stops the container.
pub struct Infra {
    pub database_url: String,
    _pg: ContainerAsync<Postgres>,
}

impl Infra {
    pub async fn start() -> Result<Self> {
        let pg = Postgres::default()
            .start()
            .await
            .context("starting Postgres container")?;
        let pg_port = pg
            .get_host_port_ipv4(5432)
            .await
            .context("resolving Postgres host port")?;
        let database_url =
            format!("postgres://{DB_USER}:{DB_PASSWORD}@127.0.0.1:{pg_port}/{DB_NAME}");

        wait_for_db(&database_url).await?;

        Ok(Self {
            database_url,
            _pg: pg,
        })
    }
}

/// Poll until Postgres accepts a real connection. The log-based readiness
/// condition can fire on the temporary initdb server, so connect for real.
async fn wait_for_db(database_url: &str) -> Result<()> {
    let mut last = String::new();
    for _ in 0..60 {
        match tokio_postgres::connect(database_url, tokio_postgres::NoTls).await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    let _ = connection.await;
                });
                if client.simple_query("SELECT 1").await.is_ok() {
                    return Ok(());
                }
            }
            Err(err) => last = err.to_string(),
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    bail!("Postgres never accepted connections at {database_url}: {last}");
}
