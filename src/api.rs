use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::Value;

use crate::config;

pub async fn list_sources(
    Extension(agent): Extension<ureq::Agent>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = format!("{}/{}", config::NPM_REGISTRY, "/source-list/latest");
    let body = agent
        .get(&url)
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_json::<Value>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let source_list = body["sources"].as_array().unwrap().to_vec();

    Ok(Json(source_list))
}

pub async fn get_lastest_build(
    Extension(agent): Extension<ureq::Agent>,
    Path((source, champion)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = format!("{}/{}/latest", config::NPM_REGISTRY, source);
    let body = agent
        .get(&url)
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_json::<Value>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let latest_version = body["version"].as_str().unwrap();
    let build_url = format!(
        "{}/{}@{}/{}.json",
        config::CDN_URL,
        source,
        latest_version,
        champion
    );
    let body = agent
        .get(&build_url)
        .call()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_string()
        // .into_json::<Value>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let data = serde_json::from_str::<Value>(&body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(data))
}

pub async fn get_latest_version() {}
