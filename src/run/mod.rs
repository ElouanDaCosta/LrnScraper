extern crate lazy_static;
extern crate num_cpus;

use std::time::Instant;

use crate::log;
use crate::utils;
use save_content::save_html_content;
use serde::{Deserialize, Serialize};
mod browser;
mod save_content;
mod thread_pool;

lazy_static::lazy_static! {
    static ref NUM_LOGICAL_CORES: usize = num_cpus::get();
    static ref MAX_THREADS: usize = *NUM_LOGICAL_CORES * 2;
    static ref MAX_WEBSITE_THREADS: usize = 2;
}

//TODO
// can scrap by html tag, css class or custom ?

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebConfigData {
    pub websites: Vec<WebConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WebConfig {
    pub id: String,
    pub name: String,
    pub save_file: String,
    pub urls: Vec<String>,
}

pub fn run_scrapper() {
    log::info_log("Getting the config file content...".to_string());
    let websites = utils::get_config_file_content();
    log::info_log("Start scraping process...".to_string());
    let pool: thread_pool::MyThreadPool = thread_pool::MyThreadPool::new(*MAX_WEBSITE_THREADS);
    for website in websites.clone() {
        pool.queue_work(Box::new(move || download_website(&website)));
    }
}

// create a new thread that scrape the html from given url
fn download_website(website: &WebConfig) {
    let start = Instant::now();
    println!("Thread started for id: {}", website.id);
    let mut crawl_url_vec: Vec<String> = Vec::new();
    for url in &website.urls {
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        crawl_url(url, "test".to_string());
        sub_thread_url(&url_clone, save_file_clone);
    }
    print!("{:?}", crawl_url_vec);
    let duration = start.elapsed().as_secs_f32();
    println!(
        "Thread finished for id: {} in {:?} secondes",
        website.id, duration
    );
}

fn sub_thread_url(url: &str, save_file: String) {
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_html_content(html_from_browser.unwrap(), "title".to_string());
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    save_html_content(parser, &save_file);
}

fn crawl_url(url: &str, save_file: String) {
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_html_content(html_from_browser.unwrap(), "a".to_string());
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    save_html_content(parser, &save_file);
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
