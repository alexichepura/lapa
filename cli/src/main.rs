use clap::{Args, Parser, Subcommand};
use config::ConfigError;
use cornucopia::{deadpool_postgres, queries};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .unwrap()
            .try_deserialize()
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    UserAdd(UserAddArgs),
    /// Disable a user account: it can no longer log in and existing sessions
    /// fail closed on their next request.
    UserDisable(UserNameArgs),
    /// Re-enable a previously disabled user account.
    UserEnable(UserNameArgs),
    /// Set a user's role (admin, editor or viewer).
    UserSetRole(UserSetRoleArgs),
    SettingsInit,
    Migrate,
}

#[derive(Args)]
struct UserAddArgs {
    username: String,
    password: String,
}

#[derive(Args)]
struct UserNameArgs {
    username: String,
}

#[derive(Args)]
struct UserSetRoleArgs {
    username: String,
    /// One of: admin, editor, viewer.
    role: String,
}

/// Set a user's role. Exits non-zero when the user does not exist and with
/// status 2 for an invalid role.
async fn set_role(pool: &deadpool_postgres::Pool, username: &str, role: &str) {
    let role = match role {
        "admin" | "editor" | "viewer" => role,
        _ => {
            eprintln!("Invalid role {role:?}; expected one of: admin, editor, viewer");
            std::process::exit(2);
        }
    };
    let db = pool.clone().get().await.unwrap();
    let updated = queries::user::user_set_role()
        .bind(&db, &role, &username)
        .await
        .unwrap();
    if updated == 0 {
        eprintln!("No user named {username:?}");
        std::process::exit(1);
    }
    println!("User {username:?} is now {role}");
}

/// Enable or disable a user by username. Exits non-zero when no such user
/// exists so the command can be scripted.
async fn set_active(pool: &deadpool_postgres::Pool, username: &str, active: bool) {
    let db = pool.clone().get().await.unwrap();
    let updated = queries::user::user_set_active()
        .bind(&db, &active, &username)
        .await
        .unwrap();
    if updated == 0 {
        eprintln!("No user named {username:?}");
        std::process::exit(1);
    }
    let state = if active { "active" } else { "disabled" };
    println!("User {username:?} is now {state}");
    if !active {
        purge_sessions(pool, username).await;
    }
}

/// Delete every session belonging to `username` (best effort, reported).
async fn purge_sessions(pool: &deadpool_postgres::Pool, username: &str) {
    let db = pool.clone().get().await.unwrap();
    match queries::session::delete_by_username()
        .bind(&db, &username)
        .await
    {
        Ok(purged) => println!("Purged {purged} session(s) for {username:?}"),
        Err(e) => eprintln!("Failed to purge sessions for {username:?}: {e}"),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();

    dotenv().expect(".env file not found");
    let config = Config::from_env().unwrap();
    let pool = lib::create_pool(&config.pg);

    match &cli.command {
        Commands::Migrate => {
            db::migrate(&pool).await.unwrap();
            println!("Migration success");
        }
        Commands::UserAdd(user) => {
            let username = user.username.clone();
            let password = user.password.clone();
            println!("username is: {:?}", username);
            let password_hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
            let db = pool.clone().get().await.unwrap();
            let res = queries::user::user_create()
                .bind(&db, &cuid2::create_id(), &username, &password_hashed)
                .await;
            println!("User created res={res:?}");
        }
        Commands::UserDisable(user) => {
            set_active(&pool, &user.username, false).await;
        }
        Commands::UserEnable(user) => {
            set_active(&pool, &user.username, true).await;
        }
        Commands::UserSetRole(user) => {
            set_role(&pool, &user.username, &user.role).await;
        }
        Commands::SettingsInit => {
            let db = pool.clone().get().await.unwrap();
            let settings = queries::settings::settings().bind(&db).opt().await.unwrap();
            if let Some(settings) = settings {
                panic!("Settings found {:?}", settings);
            }
            let res = queries::settings::settings_create()
                .bind(&db, &cuid2::create_id())
                .await;
            println!("Settings created res={res:?}");
        }
    }
}
