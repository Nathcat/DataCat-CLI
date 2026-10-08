use clap::{Parser, Subcommand};

use crate::groups::listmembers::ListMembers;

mod listmembers;

#[derive(Parser, Debug)]
#[command(about = "Manage DataCat groups.", long_about = None)]
pub struct Groups {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    ListMembers(ListMembers),
}

impl Groups {
    pub fn handle(self) {
        match self.action {
            Action::ListMembers(v) => v.list(),
        }
    }
}
