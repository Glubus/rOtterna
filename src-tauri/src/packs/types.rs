use serde::{Deserialize, Serialize};
use super::utils::deserialize_f64_from_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacksResponse {
    pub data: Vec<Pack>,
    pub links: PackLinks,
    pub meta: PackMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pack {
    pub id: u64,
    pub name: String,
    #[serde(rename = "play_count")]
    pub play_count: u64,
    #[serde(rename = "song_count")]
    pub song_count: u64,
    #[serde(rename = "banner_path")]
    pub banner_path: String,
    #[serde(rename = "bannerTinyThumb", default)]
    pub banner_tiny_thumb: String,
    #[serde(rename = "bannerSrcSet", default)]
    pub banner_src_set: String,
    #[serde(rename = "contains_nsfw")]
    pub contains_nsfw: bool,
    pub size: String,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub overall: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub stream: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub jumpstream: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub handstream: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub jacks: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub chordjacks: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub stamina: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub technical: f64,
    pub tags: Vec<Tag>,
    pub download: String,
    pub magnet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "type")]
    pub tag_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackLinks {
    pub first: String,
    pub last: String,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMeta {
    #[serde(rename = "current_page")]
    pub current_page: u64,
    pub from: u64,
    #[serde(rename = "last_page")]
    pub last_page: u64,
    pub links: Vec<PackMetaLink>,
    pub path: String,
    #[serde(rename = "per_page")]
    pub per_page: u64,
    pub to: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMetaLink {
    pub url: Option<String>,
    pub label: String,
    pub active: bool,
}

