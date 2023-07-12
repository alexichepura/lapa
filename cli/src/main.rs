use clap::{Args, Parser, Subcommand};
use prisma_client::db;
use std::sync::Arc;

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
        dbg!(&db_url);
        db::new_client_with_url(db_url.as_str()).await
    } else {
        println!("DATABASE_URL not set");
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

            let user_created = prisma_client
                .clone()
                .user()
                .create(username.clone(), password_hashed, vec![])
                .exec()
                .await;

            println!("User created: {:?}", user_created);
        }
        Commands::SettingsInit => {
            let settings_found = prisma_client
                .settings()
                .find_first(vec![])
                .select(db::settings::select!({
                    hero_height
                    hero_width
                    thumb_height
                    thumb_width
                }))
                .exec()
                .await;

            if let Ok(Some(settings_found)) = settings_found {
                panic!("Settings found {:?}", settings_found);
            }

            let settings_created = prisma_client
                .settings()
                .create(
                    480,
                    640,
                    240,
                    320,
                    vec![db::settings::robots_txt::set(
                        "User-agent: * \nDisallow: /pkg/".to_string(),
                    )],
                )
                .exec()
                .await;

            println!("Settings created: {:?}", settings_created);
        }
    }
}
