use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Sync,
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "cli", long_about = "sync ChampR builds")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let mut pg_conn = db::establish_connection();

    match &cli.command {
        Some(Commands::Sync) => {
            println!("started sync command");

            match service::list_sources().await {
                Ok(list) => {
                    let new_sources = list
                        .iter()
                        .map(|source| {
                            let name = &source.label;
                            let source = &source.value;
                            let version = "1.0.0";
                            db::models::NewSource {
                                name,
                                source,
                                version,
                            }
                        })
                        .collect::<Vec<db::models::NewSource>>();
                    let total = db::insert_many_sources(&mut pg_conn, new_sources);
                    println!("inserted: {total}");
                }
                Err(err) => {
                    println!("error: {}", err);
                }
            }
        }
        _ => {
            println!("nothing");
        }
    };

    Ok(())
}
