use std::{error::Error, process::exit};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth_client::ClientConfig,
    errors::{no_auth_client_setup, no_auth_grant_setup},
    files::{get_auth_clients, get_auth_grants, update_auth_grants},
    login::Credentials,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullName: String,
    pub email: String,
    pub pfpPath: String,
    pub verified: i8,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct UserNoEmail {
    pub id: i32,
    pub username: String,
    pub fullName: String,
    pub pfpPath: String,
    pub verified: i8,
}

#[derive(serde::Deserialize)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

pub fn request_access_token(
    auth_host: &String,
    grant: &String,
    oauth_client: &ClientConfig,
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let mut url = Url::parse(auth_host).unwrap().join("token").unwrap();
    let mut query_string = String::from("grant_type=authorization_code&code=");
    query_string.push_str(grant);

    url.set_query(Some(&query_string));

    let mut auth_header = String::from("Basic ");
    auth_header.push_str(&oauth_client.id);
    auth_header.push_str(":");
    auth_header.push_str(&oauth_client.secret);

    let response = client
        .post(url)
        .header("Authorization", auth_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send();

    if let Err(e) = response {
        Err(e)
    } else {
        let body = response.unwrap().json::<AccessToken>();

        if let Err(e) = body {
            eprintln!("{}", e.to_string());
            if let Some(e) = e.source() {
                eprintln!("source: {}", e.to_string());
            }

            exit(-1);
        } else {
            Ok(body.unwrap().access_token)
        }
    }
}

pub fn get_access_token(authcat_host: &String) -> String {
    let auth_clients = get_auth_clients();
    let oauth_client: &ClientConfig;
    match auth_clients.get(&authcat_host.to_string()) {
        None => {
            no_auth_client_setup(authcat_host.to_string());
            exit(-1);
        }
        Some(v) => oauth_client = v,
    }

    let mut auth_grants = get_auth_grants();
    let mut credentials: Credentials;
    match auth_grants.get_mut(&authcat_host.to_string()) {
        None => {
            no_auth_grant_setup(authcat_host.to_string());
            exit(-1);
        }
        Some(v) => credentials = v.clone(),
    }

    if let None = credentials.token {
        let token =
            request_access_token(&authcat_host.to_string(), &credentials.grant, oauth_client)
                .unwrap();

        credentials.token = Some(token);
        auth_grants.insert(authcat_host.to_string(), credentials.clone());
        update_auth_grants(auth_grants);
    }

    credentials.token.unwrap()
}
