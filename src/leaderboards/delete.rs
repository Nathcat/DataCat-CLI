use clap::Parser;
use reqwest::StatusCode;
use serde_json::json;
use url::Url;

use crate::{DEFAULT_DATACAT_HOST, files::get_selected_app};

#[derive(Parser, Debug)]
#[command(about = "Delete a leaderboard.", long_about = None)]
pub struct Delete {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    id: u32,
}

impl Delete {
    pub fn delete(self) {
        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join(&format!("api/leaderboards/{}", self.id.to_string()))
        .unwrap();

        let app = get_selected_app();

        println!(
            "Deleting leaderboard ID {} for app '{}'.",
            self.id, app.name
        );

        let request = reqwest::blocking::Client::new()
            .delete(datacat_host)
            .header("Authorization", format!("Basic {}:{}", app.id, app.apiKey))
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
