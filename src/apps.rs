use clap::{Parser, Subcommand};

use crate::apps::{delete::Delete, list::List, new::New};

mod delete;
mod list;
mod new;

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
}

impl Apps {
    pub fn handle(self) {
        match self.action {
            Action::List(v) => v.list(),
            Action::New(v) => v.new(),
            Action::Delete(v) => v.delete(),
        }
    }
}
