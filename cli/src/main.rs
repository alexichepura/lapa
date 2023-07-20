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
    Migrate,
    Css,
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
        Commands::Css => {
            use lightningcss::{
                bundler::{Bundler, FileProvider},
                stylesheet::{ParserFlags, ParserOptions, PrinterOptions},
                targets::Targets,
            };
            use std::{fs::File, io::prelude::*, path::Path};

            let fs = FileProvider::new();
            let parser_options = ParserOptions {
                flags: ParserFlags::NESTING | ParserFlags::CUSTOM_MEDIA,
                ..ParserOptions::default()
            };
            let mut bundler = Bundler::new(&fs, None, parser_options);
            let stylesheet = bundler.bundle(Path::new("./style/main.css")).unwrap();
            let options = PrinterOptions::<'_> {
                targets: Targets::default(),
                // targets: Targets::from(Browsers::default()),
                minify: true,
                ..Default::default()
            };
            let style_output = stylesheet.to_css(options).unwrap();
            let bytes = style_output.code.as_bytes();
            let mut buffer = File::create("./style/main-bundle.css").unwrap();
            buffer.write(bytes).unwrap();
            println!("Css success");
        }
        Commands::Migrate => {
            prisma_client._migrate_deploy().await.unwrap();
            println!("Migration success");
        }
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
