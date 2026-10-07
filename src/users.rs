use std::{default, error::Error, process::exit};

use clap::Parser;
use json_colorizer::{FormatOptions, format_json};
use reqwest::{StatusCode, blocking::Client};
use url::Url;

use crate::{
    DEFAULT_AUTHCAT_HOST,
    auth_client::ClientConfig,
    authcat::{User, UserNoEmail, get_access_token, request_access_token},
    errors::{no_auth_client_setup, no_auth_grant_setup, unauthorized},
    files::{get_auth_clients, get_auth_grants, update_auth_grants},
    login::Credentials,
};

#[derive(Parser, Debug)]
#[command(about = "Search for users", long_about = None)]
pub struct Users {
    #[arg(short, long)]
    username: Option<String>,

    #[arg(short, long)]
    id: Option<u32>,

    #[arg(short, long)]
    authcat_host: Option<String>,
}

impl Users {
    pub fn search(self) {
        let authcat_host = Url::parse(
            &self
                .authcat_host
                .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST)),
        )
        .unwrap();

        let mut user_url = authcat_host.join("user").unwrap();

        if let (None, None) = (&self.username, &self.id) {
            let access_token = get_access_token(&authcat_host.to_string());

            let client = reqwest::blocking::Client::new();
            let response = client
                .get(user_url)
                .bearer_auth(access_token)
                .send()
                .unwrap();

            match response.status() {
                StatusCode::UNAUTHORIZED => {
                    unauthorized(authcat_host.to_string());
                    exit(-1);
                }
                StatusCode::OK => {
                    println!(
                        "{}",
                        format_json(
                            &serde_json::from_str(&response.text().unwrap()).unwrap(),
                            &FormatOptions::default()
                        )
                    );
                }
                v => {
                    eprintln!("Non-standard response code: {}", v.to_string());
                }
            }
        } else {
            let mut query_string = String::from("");

            if let Some(username) = &self.username {
                query_string.push_str("username=");
                query_string.push_str(username);
            }

            if let Some(id) = self.id {
                if let Some(_) = self.username {
                    query_string.push_str("&");
                }

                query_string.push_str("id=");
                query_string.push_str(&id.to_string());
            }

            user_url.set_query(Some(&query_string));

            let response = reqwest::blocking::get(user_url);
            if let Err(e) = response {
                eprintln!("{}", e.to_string());
                if let Some(e) = e.source() {
                    eprintln!("{}", e.to_string());
                }
            } else {
                println!(
                    "{}",
                    format_json(
                        &serde_json::from_str(&response.unwrap().text().unwrap()).unwrap(),
                        &FormatOptions::default()
                    )
                );
            }
        }
    }
}
