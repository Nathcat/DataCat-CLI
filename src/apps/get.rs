use clap::Parser;
use json_colorizer::{FormatOptions, format_json};
use url::Url;

use crate::{DEFAULT_AUTHCAT_HOST, DEFAULT_DATACAT_HOST, authcat::get_access_token};

#[derive(Parser, Debug)]
#[command(about = "Get apps owned by the user.", long_about = None)]
pub struct Get {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    authcat_host: Option<String>,
}

impl Get {
    pub fn get(self) {
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

        println!(
            "{}",
            format_json(
                &serde_json::from_str(
                    &reqwest::blocking::Client::new()
                        .get(datacat_host)
                        .bearer_auth(access_token)
                        .send()
                        .unwrap()
                        .text()
                        .unwrap()
                )
                .unwrap(),
                &FormatOptions::default()
            )
        );
    }
}
