use std::{error::Error, process::exit};

use clap::{Parser, Subcommand};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use url::Url;

use crate::{
    DEFAULT_AUTHCAT_HOST,
    auth_client::ClientConfig,
    files::{add_auth_grant, get_auth_clients},
};

#[derive(Parser, Debug)]
#[command(about = "Login to an AuthCat OAuth server.", long_about = None)]
pub struct Login {
    #[arg(short, long)]
    url: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub grant: String,
    pub token: Option<String>,
}

impl Login {
    /// Entry point for login command
    pub fn login(&self) {
        let mut username = String::new();

        print!("Username > ");
        io::stdout().flush().unwrap();
        if let Err(e) = io::stdin().read_line(&mut username) {
            eprint!("{}", e.to_string());
            exit(-1);
        }

        username = String::from(username.trim());

        print!("Password > ");
        io::stdout().flush().unwrap();
        let password = readpass::from_tty().unwrap();
        self.attempt_login(username, password.to_string());
    }

    fn attempt_login(&self, username: String, password: String) {
        let str_url = self
            .url
            .clone()
            .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST));
        let clients = get_auth_clients();
        let client: &ClientConfig;
        if let Some(v) = clients.get(&str_url) {
            client = v;
        } else {
            eprintln!(
                "No auth client set up for {}. To set one up: \n\nclicat auth-client add -u {}",
                str_url, str_url
            );
            exit(-1);
        }

        let mut url = Url::parse(&str_url).unwrap();
        url = url.join("auth/form").unwrap();

        let params = [
            ("username", username),
            ("password", password),
            ("client_id", client.id.clone()),
        ];

        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        let response = client.post(url).form(&params).send();

        match response {
            Err(e) => {
                eprintln!("{}", e.to_string());
                match e.source() {
                    None => {}
                    Some(v) => eprintln!("{}", v.to_string()),
                }
                exit(-1);
            }
            Ok(v) => {
                println!("{}", v.status().to_string());

                if v.status() == StatusCode::FOUND {
                    let location = v.headers().get("Location").unwrap().to_str().unwrap();
                    let location_url = Url::parse(location).unwrap();

                    for (k, v) in location_url.query_pairs() {
                        if k == "error" {
                            eprintln!(
                                "Authentication failed! {} returned error message: {}",
                                str_url, v
                            );
                            exit(-1);
                        } else if k == "code" {
                            add_auth_grant(str_url.clone(), v.to_string());
                            println!("Done!");
                        }
                    }
                }
            }
        }
    }
}
