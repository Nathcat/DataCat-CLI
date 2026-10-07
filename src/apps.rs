use clap::{Parser, Subcommand};

use crate::apps::{list::List, new::New};

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
}

impl Apps {
    pub fn handle(self) {
        match self.action {
            Action::List(v) => v.list(),
            Action::New(v) => v.new(),
        }
    }
}
