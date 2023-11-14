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

pub fn get_sources() -> Result<Vec<Source>, ureq::Error> {
    Ok(
        ureq::get("https://cdn.jsdelivr.net/npm/@champ-r/source-list")
            .call()?
            .into_json::<Vec<Source>>()?,
    )
}
