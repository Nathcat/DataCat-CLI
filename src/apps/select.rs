use std::{
    io::{self, Write},
    process::exit,
};

use clap::Parser;
use url::Url;

use crate::{
    DEFAULT_AUTHCAT_HOST, DEFAULT_DATACAT_HOST,
    apps::App,
    authcat::get_access_token,
    files::{get_selected_app, set_selected_app},
};

#[derive(Parser, Debug)]
#[command(about = "Select an application to use for commands requiring application authentication.", long_about = None)]
pub struct Select {
    #[arg(short, long)]
    datacat_host: Option<String>,

    #[arg(short, long)]
    authcat_host: Option<String>,
}

impl Select {
    pub fn select(self) {
        let authcat_host = Url::parse(
            &self
                .authcat_host
                .unwrap_or(String::from(DEFAULT_AUTHCAT_HOST)),
        )
        .unwrap();
        let datacat_host = Url::parse(
            &self
                .datacat_host
                .unwrap_or(String::from(DEFAULT_DATACAT_HOST)),
        )
        .unwrap()
        .join("api/apps")
        .unwrap();

        let access_token = get_access_token(&authcat_host.to_string());

        let apps = reqwest::blocking::Client::new()
            .get(datacat_host)
            .bearer_auth(access_token)
            .send()
            .unwrap()
            .json::<Vec<App>>()
            .unwrap();

        println!("Select an app from the list");

        let mut i = 1;
        for app in apps.iter() {
            println!("{}: {}", i, app.name);
            i += 1;
        }

        let mut choice: i32;

        loop {
            let mut choice_str: String = String::new();
            print!(" > ");
            io::stdout().flush().unwrap();
            if let Err(e) = io::stdin().read_line(&mut choice_str) {
                eprint!("{}", e.to_string());
                exit(-1);
            }

            choice_str = String::from(choice_str.trim());

            let parse_res = choice_str.parse::<i32>();
            if let Ok(v) = parse_res {
                choice = v;

                if (choice as usize) > apps.len() || choice <= 0 {
                    println!("Choice is not within a valid range.");
                } else {
                    break;
                }
            } else {
                eprintln!("{}", parse_res.unwrap_err());
                println!("Not a valid choice, try again.");
            }
        }

        let app = apps.get((choice - 1) as usize).unwrap();
        set_selected_app(app);
        println!("Selected '{}'", app.name);
    }
}
