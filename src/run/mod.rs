use std::fs;
use std::thread;

use crate::log;
use serde::{Deserialize, Serialize};
mod browser;

//TODO
// can scrap by html tag, css class or custom ?

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
    std::thread::scope(|_scope| {
        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
        for website in websites.clone() {
            let thread = std::thread::spawn(move || download_website(&website));
            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }
    });
    log::info_log("Scraping process finished successfully.".to_string());
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
fn download_website(website: &WebConfig) {
    println!("Thread started for id: {}", website.id);
    for url in &website.urls {
        let response = reqwest::blocking::get(url);
        let html_from_request = response.unwrap().text().unwrap();
        let html_from_browser = browser::browse_website(&url);
        let parser = parse_html_content(html_from_browser.unwrap(), "title".to_string());
        println!("{:?}", parser);
    }
}

fn parse_html_content(data: String, tag_selector: String) -> Vec<String> {
    let dom = tl::parse(&data, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let elements = dom
        .query_selector(&tag_selector)
        .expect("Failed to find element");
    let mut nodes = Vec::new();
    for element in elements {
        let node = element.get(parser).unwrap();
        nodes.push(node.inner_text(parser).to_string());
    }
    nodes
}
