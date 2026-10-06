use std::{
    collections::HashMap,
    fs::{File, create_dir, create_dir_all, exists, read_to_string},
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
};

use dirs::home_dir;
use url::Url;

use crate::{
    AUTH_CLIENTS_FILE, STORAGE_LOCATION, USER_AUTH_FILE, auth_client::ClientConfig,
    login::Credentials,
};

type AuthClients = HashMap<String, ClientConfig>;
type AuthGrants = HashMap<String, Credentials>;

fn storage_dir() -> PathBuf {
    Path::new(&home_dir().unwrap()).join(STORAGE_LOCATION)
}

pub fn init_storage_dir() {
    if !exists(storage_dir()).unwrap() {
        create_dir_all(storage_dir()).unwrap();
    }
}

fn get_auth_clients_file_writable() -> File {
    return File::create(storage_dir().join(AUTH_CLIENTS_FILE)).unwrap();
}

fn init_auth_clients_file() {
    let path = storage_dir().join(AUTH_CLIENTS_FILE);
    if !exists(&path).unwrap() {
        let mut file = get_auth_clients_file_writable();
        file.write_all(
            serde_json::to_string(&AuthClients::new())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }
}

pub fn get_auth_clients() -> AuthClients {
    init_auth_clients_file();
    let content = read_to_string(storage_dir().join(AUTH_CLIENTS_FILE)).unwrap();

    return serde_json::from_str::<AuthClients>(&content).unwrap();
}

pub fn add_auth_client(client: ClientConfig, url: String) {
    let mut clients = get_auth_clients();
    clients.insert(url, client);

    let mut file = get_auth_clients_file_writable();
    file.write_all(serde_json::to_string(&clients).unwrap().as_bytes())
        .unwrap();
}

fn get_user_auth_file_writable() -> File {
    return File::create(storage_dir().join(USER_AUTH_FILE)).unwrap();
}

fn init_auth_grants_file() {
    let path = storage_dir().join(USER_AUTH_FILE);
    if !exists(&path).unwrap() {
        let mut file = get_user_auth_file_writable();
        file.write_all(
            serde_json::to_string(&AuthGrants::new())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }
}

pub fn get_auth_grants() -> AuthGrants {
    init_auth_grants_file();
    let content = read_to_string(storage_dir().join(USER_AUTH_FILE)).unwrap();

    return serde_json::from_str::<AuthGrants>(&content).unwrap();
}

pub fn add_auth_grant(url: String, code: String) {
    let mut grants = get_auth_grants();
    grants.insert(
        url,
        Credentials {
            grant: code,
            token: None,
        },
    );

    let mut file = get_user_auth_file_writable();
    file.write_all(serde_json::to_string(&grants).unwrap().as_bytes())
        .unwrap();
}

pub fn add_auth_token(url: String, token: String) {
    let mut grants = get_auth_grants();
    let creds = grants.get_mut(&url).unwrap();
    creds.token = Some(token);

    let mut file = get_user_auth_file_writable();
    file.write_all(serde_json::to_string(&grants).unwrap().as_bytes())
        .unwrap();
}
