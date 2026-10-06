use clap::{Parser, Subcommand};

use crate::auth_client::add::Add;

mod add;

#[derive(Parser, Debug)]
#[command(about = "Setup OAuth client info", long_about = None)]
pub struct AuthClient {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Add(Add),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ClientConfig {
    pub id: String,
    pub secret: String,
}

impl AuthClient {
    pub fn handle(self) {
        match self.action {
            Action::Add(v) => v.setup(),
        }
    }
}
