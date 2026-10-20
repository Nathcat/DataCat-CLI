use clap::{Parser, Subcommand};

use crate::apps::{delete::Delete, list::List, new::New, select::Select};

mod delete;
mod list;
mod new;
mod select;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct App {
    pub id: u32,
    pub owner: u32,
    pub name: String,
    pub apiKey: String,
}

#[derive(Parser, Debug)]
#[command(about = "Manage DataCat apps.", long_about = None)]
pub struct Apps {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    List(List),
    New(New),
    Delete(Delete),
    Select(Select),
}

impl Apps {
    pub fn handle(self) {
        match self.action {
            Action::List(v) => v.list(),
            Action::New(v) => v.new(),
            Action::Delete(v) => v.delete(),
            Action::Select(v) => v.select(),
        }
    }
}
