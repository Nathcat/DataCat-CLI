use clap::Parser;
use json_colorizer::{FormatOptions, format_json};
use url::Url;

use crate::DEFAULT_DATACAT_HOST;

#[derive(Parser, Debug)]
#[command(about = "List the members of a group.", long_about = None)]
pub struct ListMembers {
    #[arg(short, long)]
    id: u32,

    #[arg(short, long)]
    datacat_host: Option<String>,
}

impl ListMembers {
    pub fn list(self) {
        let mut path = String::from("api/groups/");
        path.push_str(&self.id.to_string());
        path.push_str("/members");

        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join(&path)
        .unwrap();

        println!(
            "{}",
            format_json(
                &serde_json::from_str(
                    &reqwest::blocking::get(datacat_host)
                        .unwrap()
                        .text()
                        .unwrap()
                )
                .unwrap(),
                &FormatOptions::default()
            )
        );
    }
}
