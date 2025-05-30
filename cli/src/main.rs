use clap::{Args, Parser, Subcommand};
use clorinde::queries;

mod db;

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
    SettingsInit,
    Migrate,
}

#[derive(Args)]
struct UserAddArgs {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();

    let pool = db::create_pool().await.unwrap();

    match &cli.command {
        Commands::Migrate => {
            // prisma_client._migrate_deploy().await.unwrap();
            // println!("Migration success");
        }
        Commands::UserAdd(user) => {
            let username = user.username.clone();
            let password = user.password.clone();
            println!("username is: {:?}", &username);
            let password_hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
            let db = pool.clone().get().await.unwrap();
            let res = queries::user::user_create()
                .bind(&db,&username, &password_hashed)
                .await;
            println!("User created res={res:?}");
        }
        Commands::SettingsInit => {
            let db = pool.clone().get().await.unwrap();
            let settings = queries::settings::settings().bind(&db).opt().await.unwrap();
            if let Some(settings) = settings {
                panic!("Settings found {:?}", settings);
            }
            let res = queries::settings::settings_create()
                .bind(&db,&480,&640,&240,&320)
                .await;
            println!("Settings created res={res:?}");
        }
    }
}
