use clap::Parser;
use json_colorizer::{FormatOptions, format_json};
use reqwest::StatusCode;
use serde_json::json;
use url::Url;

use crate::{DEFAULT_AUTHCAT_HOST, DEFAULT_DATACAT_HOST, authcat::get_access_token};

#[derive(Parser, Debug)]
#[command(about = "Get apps owned by the user.", long_about = None)]
pub struct New {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    authcat_host: Option<String>,

    #[arg(short, long)]
    name: String,
}

struct Body {
    name: String,
}

impl New {
    pub fn new(self) {
        let authcat_host = Url::parse(
            &self
                .authcat_host
                .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST)),
        )
        .unwrap();
        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join("api/apps")
        .unwrap();

        let access_token = get_access_token(&authcat_host.to_string());

        let client = reqwest::blocking::Client::new();
        let response = client
            .put(datacat_host.clone())
            .bearer_auth(access_token)
            .json(&json!({"name": self.name}))
            .send()
            .unwrap();

        if response.status() == StatusCode::OK {
            println!("Done!");
        } else {
            eprintln!(
                "{} responded with {}",
                datacat_host.to_string(),
                response.status().to_string()
            );
        }
    }
}
