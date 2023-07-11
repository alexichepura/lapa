use std::sync::Arc;

use clap::{Args, Parser, Subcommand};
use prisma_client::db;

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
}

#[derive(Args)]
struct UserAddArgs {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = if let Ok(db_url) = std::env::var("DATABASE_URL") {
        db::new_client_with_url(db_url.as_str()).await
    } else {
        db::new_client().await
    };
    let prisma_client = Arc::new(client.unwrap());
    #[cfg(debug)]
    prisma_client._db_push(false).await.unwrap();

    match &cli.command {
        Commands::UserAdd(user) => {
            let username = user.username.clone();
            let password = user.password.clone();

            println!("username is: {:?}", &username);

            let password_hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

            let result = prisma_client
                .clone()
                .user()
                .create(username.clone(), password_hashed, vec![])
                .exec()
                .await;

            dbg!(result);
        }
    }
}
