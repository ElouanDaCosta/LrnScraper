use std::fs;
use std::thread;

use crate::log;
use save_content::save_html_content;
use serde::{Deserialize, Serialize};
mod browser;
mod save_content;

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
    save_file: String,
    urls: Vec<String>,
}

pub fn run_scrapper() {
    log::info_log("Getting the config file content...".to_string());
    let websites = get_config_file_content();
    log::info_log("Start scraping process...".to_string());
    std::thread::scope(|_scope| {
        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
        let mut save_file: Vec<String> = Vec::new();
        for website in websites.clone() {
            let save_file_clone = website.save_file.clone();
            let thread = std::thread::spawn(move || download_website(&website));
            save_file.push(save_file_clone);
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
            save_file: website.save_file.clone(),
            urls: website.urls.clone(),
        });
    }
    websites
}

// create a new thread that scrape the html from given url
fn download_website(website: &WebConfig) {
    println!("Thread started for id: {}", website.id);
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for url in &website.urls {
        // let response = reqwest::blocking::get(url);
        // let html_from_request = response.unwrap().text().unwrap();
        let id_clone = website.id.clone();
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        let thread =
            std::thread::spawn(move || sub_thread_url(&url_clone, id_clone, save_file_clone));
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!("Thread finished for id: {}", website.id);
}

fn sub_thread_url(url: &str, id: String, save_file: String) {
    println!("Sub-Thread started for id: {}", id);
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_html_content(html_from_browser.unwrap(), "title".to_string());
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content from id:".to_string(),
            id.to_string(),
        );
    }
    save_html_content(parser, &save_file);
    println!("Sub-Thread finished for id: {}", id);
}

// parse the given html document
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
