use std::{env, process::exit};
mod clean;
mod init;
pub mod log;
mod run;
pub mod template;
pub mod utils;

const VERSION: &'static str = "0.5.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.iter().last() {
        match arg.as_str().trim() {
            "-v" => {
                utils::command_usage(&rustyspider_version());
            }
            "--version" => {
                utils::command_usage(&rustyspider_version());
            }
            _ => {}
        }
    }

    match args.get(1).map(|s| s.as_str()) {
        Some("init") => init::init_config(),
        Some("run") => run::run_scrapper(),
        Some("clean") => clean::clean_data_folder(),
        Some("help") => utils::rustyspider_usage(),
        _ => {
            usage_and_exit("Invalid command".to_string());
            return;
        }
    };
}

fn usage_and_exit(msg: String) {
    if msg != "" {
        eprintln!("{}", msg);
    }

    utils::rustyspider_usage();

    exit(0);
}

pub fn rustyspider_version() -> String {
    let usage = format!("rustyspider {VERSION}");
    usage
}
