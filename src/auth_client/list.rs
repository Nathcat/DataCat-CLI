use json_colorizer::{FormatOptions, format_json};

use crate::files::get_auth_clients;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "List auth clients.", long_about = None)]
pub struct List {}

impl List {
    pub fn list(self) {
        let clients = get_auth_clients();
        println!(
            "{}",
            format_json(
                &serde_json::to_value(&clients).unwrap(),
                &FormatOptions::default()
            )
        )
    }
}
