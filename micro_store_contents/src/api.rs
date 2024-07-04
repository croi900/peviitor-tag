use crate::caching::update_jobs_html;
use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};
use std::{fs, net::SocketAddr};
#[derive(Debug, Deserialize)]
pub struct SourceParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub id: Option<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub async fn get_source(Query(params): Query<SourceParams>) -> String {
    let res = match params.id {
        Some(id) => {
            fs::read_to_string(format!("job_sources/{}.txt", id)).unwrap_or("nu exista".to_string())
        }
        None => "ma-ta".to_string(),
    };
    res
}

#[derive(Debug, Deserialize)]
pub struct UpdateParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub key: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub beg: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub size: Option<String>,
}

pub async fn update(Query(params): Query<UpdateParams>) -> String {
    let res = match params.key {
        Some(key) => {
            let update_res = update_jobs_html(1000, 100, &key).await;
            match update_res {
                Ok(_) => "succes".to_string(),
                Err(_) => "failure".to_string(),
            }
        }
        None => "bad key".to_string(),
    };
    res
}
