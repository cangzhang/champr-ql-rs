use clap::{Parser, Subcommand};
use kv_log_macro as log;

#[derive(Subcommand)]
enum Commands {
    SyncSource,
    SyncBuild,
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
    use log::*;

    femme::with_level(femme::LevelFilter::Trace);

    let cli = Cli::parse();
    let mut pg_conn = db::establish_connection();

    match &cli.command {
        Some(Commands::SyncSource) => {
            info!("started sync sources");

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
                    info!("inserted: {total}");
                }
                Err(err) => {
                    error!("error: {}", err);
                }
            }
        }
        Some(Commands::SyncBuild) => {
            info!("started sync builds");

            match service::list_all_champions().await {
                Ok(resp) => {
                    info!("version {}, total: {}", resp.version, resp.data.len());
                }
                Err(e) => {
                    error!("error: {}", e);
                }
            }
        }
        _ => {
            log::info!("nothing");
        }
    };

    Ok(())
}
