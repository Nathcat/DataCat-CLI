use clap::{Parser, Subcommand};

use crate::apps::get::Get;

mod get;

#[derive(Parser, Debug)]
#[command(about = "Manage DataCat apps.", long_about = None)]
pub struct Apps {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Get(Get),
}

impl Apps {
    pub fn handle(self) {
        match self.action {
            Action::Get(v) => v.get(),
        }
    }
}
