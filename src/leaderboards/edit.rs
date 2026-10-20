use clap::Parser;
use reqwest::StatusCode;
use serde_json::{Value, json};
use url::Url;

use crate::{DEFAULT_DATACAT_HOST, files::get_selected_app};

#[derive(Parser, Debug)]
#[command(about = "Edit a leaderboard record.", long_about = None)]
pub struct Edit {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(long)]
    id: u32,

    #[arg(short, long)]
    user: u32,

    #[arg(short, long)]
    increment: Option<bool>,

    #[arg(short, long)]
    value: Option<i32>,
}

#[derive(serde::Serialize)]
struct Body {
    user: u32,
    increment: Option<bool>,
    value: Option<i32>,
}

impl Edit {
    pub fn edit(self) {
        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join(&format!("api/leaderboards/{}", self.id.to_string()))
        .unwrap();

        let app = get_selected_app();

        println!("Editing leaderboard ID {} for app '{}'.", self.id, app.name);

        let body = Body {
            user: self.user,
            increment: self.increment,
            value: self.value,
        };

        let request = reqwest::blocking::Client::new()
            .post(datacat_host)
            .header("Authorization", format!("Basic {}:{}", app.id, app.apiKey))
            .json(&body)
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
