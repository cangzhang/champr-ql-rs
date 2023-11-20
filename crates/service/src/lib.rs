use std::collections::HashMap;
use std::io::{self, Cursor};

use anyhow::Context;
use flate2::read::GzDecoder;
use futures::future::join_all;
use kv_log_macro as log;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use tar::Archive;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub label: String,
    pub value: String,
    pub is_aram: Option<bool>,
    #[serde(rename = "isURF")]
    pub is_urf: Option<bool>,
}

pub async fn list_sources() -> Result<Vec<Source>, reqwest::Error> {
    let r = reqwest::get("https://cdn.jsdelivr.net/npm/@champ-r/source-list").await?;
    r.json::<Vec<Source>>().await
}

pub async fn list_lol_versions() -> Result<Vec<String>, reqwest::Error> {
    let r = reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json").await?;
    r.json::<Vec<String>>().await
}

pub async fn get_latest_version() -> Result<String, reqwest::Error> {
    let versions = list_lol_versions().await?;
    Ok(versions.first().unwrap().to_string())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMapResp {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
}

pub async fn list_all_champions() -> Result<ChampionMapResp, reqwest::Error> {
    let version = get_latest_version().await?;

    let r = reqwest::get(format!(
        "http://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json"
    ))
    .await?;
    r.json::<ChampionMapResp>().await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dist {
    pub tarball: String,
    pub file_count: i64,
    pub unpacked_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source_version: String,
    pub description: String,
    pub dist: Dist,
}

pub async fn get_remote_package_data(source: &String) -> Result<(String, String), reqwest::Error> {
    let r = reqwest::get(format!(
        "https://registry.npmjs.org/@champ-r/{source}/latest"
    ))
    .await?;
    let pak = r.json::<Package>().await?;
    Ok((pak.version, pak.dist.tarball))
}

pub async fn download_and_extract_tgz(url: &str, output_dir: &str) -> io::Result<()> {
    // Download the file
    let response = reqwest::get(url).await.unwrap();
    let content = response.bytes().await.unwrap();
    // Cursor allows us to read bytes as a stream
    let cursor = Cursor::new(content);
    // Decompress gzip
    let gz = GzDecoder::new(cursor);
    // Extract tarball
    let mut archive = Archive::new(gz);
    archive.unpack(output_dir)?;

    Ok(())
}

pub async fn read_local_build_file(file_path: String) -> anyhow::Result<Value> {
    let mut file = File::open(&file_path)
        .await
        .with_context(|| format!("Failed to open file: {}", &file_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .with_context(|| format!("Failed to read from file: {}", &file_path))?;
    let parsed = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON in file: {}", &file_path))?;

    Ok(parsed)
}

pub async fn read_from_local_folder(output_dir: &str) -> anyhow::Result<Vec<Vec<Build>>> {
    use log::*;

    let paths = std::fs::read_dir(output_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file() && entry.file_name() != "package.json")
        .map(|entry| entry.path().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>();
    let tasks: Vec<_> = paths
        .into_iter()
        .map(|p| read_local_build_file(p.clone()))
        .collect();
    let results = join_all(tasks).await;

    let files = results
        .into_iter()
        .filter_map(|result| match result {
            Ok(value) => match serde_json::from_value::<Vec<Build>>(value) {
                Ok(builds) => Some(builds),
                Err(e) => {
                    warn!("parsing builds: {}", e);
                    None
                }
            },
            Err(e) => {
                warn!("Error: {:?}", e);
                None
            }
        })
        .collect();

    Ok(files)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub index: i64,
    pub id: String,
    pub version: String,
    pub official_version: String,
    pub pick_count: i64,
    pub win_rate: String,
    pub timestamp: i64,
    pub alias: String,
    pub name: String,
    pub position: String,
    pub skills: Option<Vec<String>>,
    pub spells: Option<Vec<String>>,
    pub item_builds: Vec<ItemBuild>,
    pub runes: Vec<Rune>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemBuild {
    pub title: String,
    pub associated_maps: Vec<i64>,
    pub associated_champions: Vec<i64>,
    pub blocks: Vec<Block>,
    pub map: String,
    pub mode: String,
    pub preferred_item_slots: Option<Vec<serde_json::Value>>,
    pub sortrank: i64,
    pub started_from: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Option<Vec<Item>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub count: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub alias: String,
    pub name: String,
    pub position: String,
    pub pick_count: u64,
    pub win_rate: String,
    pub primary_style_id: u64,
    pub sub_style_id: u64,
    pub selected_perk_ids: Vec<u64>,
    pub score: Option<f64>,
    #[serde(rename = "type", default = "empty_rune_type")]
    pub type_field: String,
}

pub fn empty_rune_type() -> String {
    String::new()
}

pub async fn get_champion_build(
    champion: String,
    source: String,
    version: String,
) -> Result<Vec<Build>, reqwest::Error> {
    let url = format!("https://cdn.jsdelivr.net/npm/@champ-r/{source}@{version}/{champion}.json");
    let resp = reqwest::get(&url).await?;
    resp.json::<Vec<Build>>().await
}
