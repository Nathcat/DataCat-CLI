use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Manage DataCat apps.", long_about = None)]
pub struct Apps {
    #[command(subcommand)]
    action: Action,
}

#[derive(Parser, Debug)]
#[command(about = "Get apps owned by the user.", long_about = None)]
struct Get {}

#[derive(Subcommand, Debug)]
enum Action {
    Get(Get),
}
