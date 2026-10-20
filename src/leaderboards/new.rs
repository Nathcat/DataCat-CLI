use clap::Parser;
use reqwest::StatusCode;
use serde_json::json;
use url::Url;

use crate::{DEFAULT_DATACAT_HOST, files::get_selected_app};

#[derive(Parser, Debug)]
#[command(about = "Create a new leaderboard for the selected app.", long_about = None)]
pub struct New {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    name: String,
}

impl New {
    pub fn new(self) {
        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join("api/leaderboards")
        .unwrap();

        let app = get_selected_app();

        println!(
            "Creating new leaderboard '{}' for app '{}'.",
            self.name, app.name
        );

        let request = reqwest::blocking::Client::new()
            .put(datacat_host)
            .header("Authorization", format!("Basic {}:{}", app.id, app.apiKey))
            .json(&json!({"name": &self.name}))
            .send();

        match request {
            Err(e) => eprintln!("{}", e.to_string()),
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    println!("Done!");
                } else {
                    eprintln!("Server responded with {}", r.status().to_string());
                }
            }
        }
    }
}
