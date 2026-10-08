use clap::Parser;
use reqwest::StatusCode;
use url::Url;

use crate::{DEFAULT_AUTHCAT_HOST, DEFAULT_DATACAT_HOST, authcat::get_access_token};

#[derive(Parser, Debug)]
#[command(about = "Delete a group.", long_about = None)]
pub struct Delete {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    authcat_host: Option<String>,

    #[arg(short, long)]
    id: u32,
}

impl Delete {
    pub fn delete(self) {
        let authcat_host = Url::parse(
            &self
                .authcat_host
                .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST)),
        )
        .unwrap();

        let mut path = String::from("api/groups/");
        path.push_str(&self.id.to_string());

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
            .delete(datacat_host.clone())
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
