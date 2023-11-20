use db::models::NewBuild;
use futures::future::join_all;

use clap::{Parser, Subcommand};
use kv_log_macro as log;
use serde_json::value::to_value;

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
async fn main() -> anyhow::Result<()> {
    use log::*;

    femme::with_level(femme::LevelFilter::Info);

    let cli = Cli::parse();
    let mut pg_conn = db::establish_connection().await?;

    let source_list = service::list_sources().await?;

    match &cli.command {
        Some(Commands::SyncSource) => {
            info!("started sync sources");

            let new_sources = source_list
                .iter()
                .map(|source| {
                    let name = source.label.clone();
                    let source = source.value.clone();
                    let version = String::from("1.0.0");
                    db::models::NewSource {
                        name,
                        source,
                        version,
                    }
                })
                .collect::<Vec<db::models::NewSource>>();
            let total = db::insert_many_sources(&mut pg_conn, new_sources).await?;
            info!("inserted: {total}");

            Ok(())
        }
        Some(Commands::SyncBuild) => {
            info!("started sync builds");
            let champion_map_resp = service::list_all_champions().await?;
            info!(
                "version {}, total: {}",
                champion_map_resp.version,
                champion_map_resp.data.len()
            );

            let champion_map = champion_map_resp.data.clone();
            for item in source_list.iter() {
                let source = item.value.clone();
                let latest_version = service::get_remote_source_version(&source).await?;
                let source_version = latest_version.clone();
                log::info!("[{}] latest version: {}", &source, &latest_version);

                let tasks = champion_map
                    .iter()
                    .map(move |(_id, champ)| {
                        service::get_champion_build(
                            champ.id.clone(),
                            source.clone(),
                            latest_version.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                let result = join_all(tasks).await;
                let mut files: Vec<&Vec<service::Build>> = vec![];
                result.iter().for_each(|r| match r {
                    Ok(builds) => files.push(builds),
                    Err(e) => {
                        log::warn!("fetch champion build error: {:?}", e);
                    }
                });

                let mut new_builds = vec![];
                for builds in files.iter() {
                    let first_build = builds.first().unwrap();

                    new_builds.push(NewBuild {
                        source: item.value.clone(),
                        version: source_version.clone(),
                        champion_id: first_build.id.clone(),
                        champion_alias: first_build.alias.clone(),
                        content: to_value(builds).unwrap(),
                    });
                }

                let ret = db::upsert_many_builds(&mut pg_conn, new_builds).await?;
                log::info!("[{}] inserted builds: {ret}", &item.value);
            }

            Ok(())
        }
        _ => {
            log::info!("no command found");
            Ok(())
        }
    }
}
