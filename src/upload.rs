use std::path::PathBuf;
use reqwest::{Client, StatusCode};
use crate::{
    error::Error,
    config::Config
};
use serde::Deserialize;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config = Config::load();
}

static APP_USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Deserialize)]
pub struct ApiUploadResponse {
    pub status: u16,
    pub id: String,
    pub password: String,
    pub delete_password: String,
    pub download_url: String
}

#[derive(Deserialize)]
pub struct ApiError {
    status: u16,
    message: String
}

// Result<URL, ErrorMsg>
pub async fn upload(filename: PathBuf) -> Result<String, Error> {

    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let basename = match filename.components().last().unwrap() {
        std::path::Component::Normal(b) => Some(b.to_string_lossy().to_string()),
        _ => None
    };

    let mime = mime_guess::from_path(&filename).first().map(|m| m.to_string()).unwrap_or_else(|| {
        match basename.as_ref().unwrap_or(&String::new()).starts_with(".") {
            true => String::from("text/plain"),
            false => String::from("application/octet-stream")
        }
    });

    let body = std::fs::read(&filename)?;

    let mut req = client.post(&CONFIG.server)
        .body(body)
        .header("Content-Type", mime);

    if let Some(b) = basename {
        req = req.query(&[("filename", &b)]);
    }

    let res = req.send().await?;

    match res.status() == StatusCode::CREATED {

        true => {

            let data: ApiUploadResponse = res.json().await?;
            Ok(data.download_url)

        },

        false => {

            let data: ApiError = res.json().await?;
            Err(Error { cause: format!("{} ({})", data.message, data.status) })

        }

    }

}
