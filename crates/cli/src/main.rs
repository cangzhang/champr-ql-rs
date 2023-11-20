use std::fs;
use std::path::Path;
use db::models::NewBuild;

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

            for item in source_list.iter() {
                let source = item.value.clone();
                let (latest_version, tarball_url) = match service::get_remote_package_data(&source).await {
                    Ok(r) => r,
                    Err(e) => {
                        error!("get remote package data failed from {}, {}", &source, e);
                        continue;
                    }
                };
                let source_version = latest_version.clone();
                info!("[{}] latest version: {}, ready to download: {}", &source, &latest_version, &tarball_url);

                let output_dir = format!("./output/{}", &source);
                let output_path = Path::new(&output_dir);
                if output_path.exists() {
                    match fs::remove_dir_all(output_path) {
                        Ok(_) => info!("removed {output_dir}"),
                        Err(e) => error!("Error removing {output_dir}: {}", e),
                    }
                }
                if let Err(e) = fs::create_dir_all(&output_path) {
                    error!("create {output_dir} failed: {}", e);
                };
                if let Err(e) = service::download_and_extract_tgz(&tarball_url, &output_dir).await {
                    error!("download & extract failed from {}, {}", &tarball_url, e);
                    continue;
                }
                info!("downloaded {tarball_url}");

                let extracted_dir = format!("{}/package", &output_dir);
                let files = service::read_from_local_folder(&extracted_dir).await?;
                let new_builds = files.iter().map(|builds| {
                    let first_build = builds.first().unwrap();
                    NewBuild {
                        source: item.value.clone(),
                        version: source_version.clone(),
                        champion_id: first_build.id.clone(),
                        champion_alias: first_build.alias.clone(),
                        content: to_value(builds).unwrap(),
                    }
                }).collect();

                let ret = db::upsert_many_builds(&mut pg_conn, new_builds).await?;
                info!("[{}] inserted builds: {ret}", &item.value);
                break;
            }

            Ok(())
        }
        _ => {
            info!("no command found");
            Ok(())
        }
    }
}
