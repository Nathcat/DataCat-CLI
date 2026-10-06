use std::{
    io::{self, Write},
    process::exit,
};

use clap::Parser;

use crate::{DEFAULT_AUTHCAT_HOST, auth_client::ClientConfig, files::add_auth_client};

#[derive(Parser, Debug)]
#[command(about = "Add a new auth client.", long_about = None)]
pub struct Add {
    #[arg(short, long)]
    url: Option<String>,
}

impl Add {
    pub fn setup(&self) {
        let mut client_id = String::new();

        print!("Client ID > ");
        io::stdout().flush().unwrap();
        if let Err(e) = io::stdin().read_line(&mut client_id) {
            eprint!("{}", e.to_string());
            exit(-1);
        }

        client_id = String::from(client_id.trim());

        print!("Secret > ");
        io::stdout().flush().unwrap();
        let secret = readpass::from_tty().unwrap();

        let conf = ClientConfig {
            id: client_id,
            secret: secret.to_string(),
        };

        add_auth_client(
            conf,
            self.url
                .clone()
                .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST)),
        );
    }
}
