use clap::Parser;
use json_colorizer::{FormatOptions, format_json};
use reqwest::StatusCode;
use url::Url;

use crate::{DEFAULT_DATACAT_HOST, files::get_selected_app};

#[derive(Parser, Debug)]
#[command(about = "Get the records of a leaderboard.", long_about = None)]
pub struct Get {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    id: u32,

    #[arg(short, long)]
    ascending: Option<bool>,

    #[arg(short, long)]
    count: Option<u32>,
}

impl Get {
    pub fn get(self) {
        let mut url = format!("api/leaderboards/{}", self.id.to_string());

        if let Some(v) = self.ascending {
            url.push_str("?ascending=");
            url.push_str(&v.to_string());
        }

        if let Some(v) = self.count {
            if let None = self.ascending {
                url.push_str("?");
            } else {
                url.push_str("&");
            }

            url.push_str("limit=");
            url.push_str(&v.to_string());
        }

        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join(&url)
        .unwrap();

        let request = reqwest::blocking::Client::new().get(datacat_host).send();

        match request {
            Err(e) => eprintln!("{}", e.to_string()),
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    println!(
                        "{}",
                        format_json(
                            &serde_json::from_str(&r.text().unwrap()).unwrap(),
                            &FormatOptions::default()
                        )
                    )
                } else {
                    eprintln!("Server responded with {}", r.status().to_string());
                }
            }
        }
    }
}
