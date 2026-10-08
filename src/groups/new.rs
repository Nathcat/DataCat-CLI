use clap::Parser;
use reqwest::StatusCode;
use serde_json::json;
use url::Url;

use crate::{DEFAULT_AUTHCAT_HOST, DEFAULT_DATACAT_HOST, authcat::get_access_token};

#[derive(Parser, Debug)]
#[command(about = "Create a new group.", long_about = None)]
pub struct New {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    authcat_host: Option<String>,

    #[arg(short, long)]
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
        .join("api/groups")
        .unwrap();

        let access_token = get_access_token(&authcat_host.to_string());

        let response = reqwest::blocking::Client::new()
            .put(datacat_host.clone())
            .json(&json!({"name": &self.name}))
            .bearer_auth(access_token)
            .send()
            .unwrap();

        if response.status() == StatusCode::OK {
            println!("Done!");
        } else {
            eprintln!(
                "{} responded with {}",
                datacat_host,
                response.status().to_string()
            );
        }
    }
}
