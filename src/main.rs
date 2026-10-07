use clap::{Parser, Subcommand};

use crate::{
    apps::Apps, auth_client::AuthClient, files::init_storage_dir, login::Login, users::Users,
};

mod apps;
mod auth_client;
mod authcat;
mod errors;
mod files;
mod login;
mod users;

static STORAGE_LOCATION: &'static str = ".local/share/clicat";
static AUTH_CLIENTS_FILE: &'static str = "auth_clients.json";
static DEFAULT_AUTHCAT_HOST: &'static str = "https://auth.nathcat.net/";
static DEFAULT_DATACAT_HOST: &'static str = "https://data.nathcat.net/";
static USER_AUTH_FILE: &'static str = "user_auth.json";

#[derive(Parser, Debug)]
#[command(about = "CLI tool for interacting with the DataCat API.")]
struct Cli {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Login(Login),
    AuthClient(AuthClient),
    Apps(Apps),
    Users(Users),
}

fn main() {
    let cli = Cli::parse();

    init_storage_dir();

    match cli.subcommand {
        Command::Login(v) => v.login(),
        Command::AuthClient(v) => v.handle(),
        Command::Apps(v) => v.handle(),
        Command::Users(v) => v.search(),
    }
}
