use clap::{Parser, Subcommand};

use crate::{auth_client::AuthClient, files::init_storage_dir, login::Login};

mod auth_client;
mod files;
mod login;

static STORAGE_LOCATION: &'static str = ".local/share/clicat";
static AUTH_CLIENTS_FILE: &'static str = "auth_clients.json";
static DEFAULT_AUTHCAT_HOST: &'static str = "https://auth.nathcat.net";
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
}

fn main() {
    let cli = Cli::parse();

    init_storage_dir();

    match cli.subcommand {
        Command::Login(v) => v.login(),
        Command::AuthClient(v) => v.handle(),
    }
}
