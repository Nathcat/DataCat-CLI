use clap::{Parser, Subcommand};

use crate::groups::{delete::Delete, list::List, listmembers::ListMembers, new::New};

mod delete;
mod list;
mod listmembers;
mod new;

#[derive(Parser, Debug)]
#[command(about = "Manage DataCat groups.", long_about = None)]
pub struct Groups {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    ListMembers(ListMembers),
    New(New),
    List(List),
    Delete(Delete),
}

impl Groups {
    pub fn handle(self) {
        match self.action {
            Action::ListMembers(v) => v.list(),
            Action::New(v) => v.new(),
            Action::List(v) => v.list(),
            Action::Delete(v) => v.delete(),
        }
    }
}
