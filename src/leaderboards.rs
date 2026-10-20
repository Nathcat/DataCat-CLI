use clap::{Parser, Subcommand};

mod delete;
mod edit;
mod get;
mod new;

#[derive(Parser, Debug)]
#[command(about = "Interact with the leaderboard API", long_about = None)]
pub struct Leaderboards {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    New(new::New),
    Delete(delete::Delete),
    Get(get::Get),
    Edit(edit::Edit),
}

impl Leaderboards {
    pub fn handle(self) {
        match self.action {
            Action::New(v) => v.new(),
            Action::Delete(v) => v.delete(),
            Action::Get(v) => v.get(),
            Action::Edit(v) => v.edit(),
        }
    }
}
