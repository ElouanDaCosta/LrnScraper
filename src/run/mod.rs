use std::fs;
use std::thread;

use crate::log;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebConfigData {
    websites: Vec<WebConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WebConfig {
    id: String,
    name: String,
    urls: Vec<String>,
}

pub fn run_scrapper() {
    log::info_log("Getting the config file content...".to_string());
    let websites = get_config_file_content();
    log::info_log("Start scraping process...".to_string());
    // let urls: Vec<Vec<String>> = websites.iter().map(|x| x.urls.clone()).collect();
    // create_thread();

    let threads: Vec<_> = websites.iter().map(|i| download_thread(i)).collect();
    for handle in threads {
        handle.join().unwrap();
    }
}

fn get_config_file_content() -> Vec<WebConfig> {
    let json_data =
        fs::read_to_string("rustyspider_config.json").expect("Failed to read config file data");
    let data: WebConfigData = serde_json::from_str(&json_data).expect("Invalid JSON");
    let mut websites: Vec<WebConfig> = Vec::new();
    for website in &data.websites {
        websites.push(WebConfig {
            id: website.id.clone(),
            name: website.name.clone(),
            urls: website.urls.clone(),
        });
    }
    websites
}

// create a new thread that will download the html from given url
fn download_thread(website: &WebConfig) -> thread::JoinHandle<()> {
    let website_clone = website.clone();
    let thread = thread::spawn(move || {
        // thread code here
        for url in website_clone.urls {
            println!("{}", url);
        }
    });

    thread
}
