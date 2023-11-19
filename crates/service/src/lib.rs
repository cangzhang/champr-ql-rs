use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    reqwest::get("https://cdn.jsdelivr.net/npm/@champ-r/source-list")
        .await
        .unwrap()
        .json::<Vec<Source>>()
        .await
}

pub async fn list_lol_versions() -> Result<Vec<String>, reqwest::Error> {
    reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json")
        .await
        .unwrap()
        .json::<Vec<String>>()
        .await
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

    reqwest::get(format!(
        "http://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json"
    ))
    .await
    .unwrap()
    .json::<ChampionMapResp>()
    .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source_version: String,
    pub description: String,
}

pub async fn get_remote_source_version(source: &String) -> Result<String, reqwest::Error> {
    let pak = reqwest::get(format!(
        "https://registry.npmjs.org/@champ-r/{source}/latest"
    ))
    .await
    .unwrap()
    .json::<Package>()
    .await?;
    Ok(pak.version)
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
    let url = format!(
        "https://cdn.jsdelivr.net/npm/@champ-r/{source}@{version}/{champion}.json"
    );

    reqwest::get(&url).await.unwrap().json::<Vec<Build>>().await
}
