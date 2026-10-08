use clap::Parser;
use reqwest::StatusCode;
use serde_json::json;
use url::Url;

use crate::{DEFAULT_AUTHCAT_HOST, DEFAULT_DATACAT_HOST, authcat::get_access_token};

#[derive(Parser, Debug)]
#[command(about = "Invite a user to a group.", long_about = None)]
pub struct Invite {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    authcat_host: Option<String>,

    #[arg(short, long)]
    group_id: u32,

    #[arg(short, long)]
    username: String,
}

impl Invite {
    pub fn invite(self) {
        let authcat_host = Url::parse(
            &self
                .authcat_host
                .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST)),
        )
        .unwrap();

        let mut path = String::from("api/groups/");
        path.push_str(&self.group_id.to_string());
        path.push_str("/invite");

        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join(&path)
        .unwrap();

        let access_token = get_access_token(&authcat_host.to_string());
        let response = reqwest::blocking::Client::new()
            .put(datacat_host.clone())
            .bearer_auth(access_token)
            .json(&json!({"username": &self.username}))
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
