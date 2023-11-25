use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::Value;

use crate::{config, errors::CustomError};

pub async fn list_sources(
    Extension(pool): Extension<db::DbPool>,
    // Extension(agent): Extension<ureq::Agent>,
) -> Result<impl IntoResponse, CustomError> {
    let sources = db::list_sources(pool).await?;
    Ok(Json(sources))
}

pub async fn get_builds_by_alias(
    Extension(pool): Extension<db::DbPool>,
    // Extension(agent): Extension<ureq::Agent>,
    Path((source, champion)): Path<(String, String)>,
) -> Result<impl IntoResponse, CustomError> {
    let b = db::find_builds_by_champion_alias_and_source(pool, champion, source).await?;
    Ok(Json(b))
}

pub async fn get_builds_by_champion_id(
    Extension(pool): Extension<db::DbPool>,
    // Extension(agent): Extension<ureq::Agent>,
    Path((source, champion_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, CustomError> {
    let b = db::find_builds_by_champion_id_and_source(pool, champion_id, source).await?;
    Ok(Json(b))
}

pub async fn get_latest_lol_version(agent: &ureq::Agent) -> Result<String, StatusCode> {
    let version_list_url = format!("{}/api/versions.json", config::DATA_DRAGON_URL);
    let body = agent
        .get(&version_list_url)
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_json::<Vec<String>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let latest_version = body.first().unwrap().to_string();

    Ok(latest_version)
}

pub async fn list_champion_map(
    Extension(agent): Extension<ureq::Agent>,
) -> Result<impl IntoResponse, StatusCode> {
    let latest_version = get_latest_lol_version(&agent).await?;

    let champion_map_url = format!(
        "{}/cdn/{latest_version}/data/en_US/champion.json",
        config::DATA_DRAGON_URL
    );
    let body = agent
        .get(&champion_map_url)
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_json::<Value>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let data = body["data"].to_string();
    let champion_map =
        serde_json::from_str::<Value>(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(champion_map))
}

pub async fn list_runes_reforged(
    Extension(agent): Extension<ureq::Agent>,
) -> Result<impl IntoResponse, StatusCode> {
    let latest_version = get_latest_lol_version(&agent).await?;

    let runes_url = format!(
        "{}/cdn/{latest_version}/data/en_US/runesReforged.json",
        config::DATA_DRAGON_URL
    );
    let body = agent
        .get(&runes_url)
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_json::<Value>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(body))
}
