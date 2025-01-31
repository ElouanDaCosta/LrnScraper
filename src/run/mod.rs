extern crate num_cpus;

use std::time::Instant;

use crate::log;
use crate::utils;
use save_content::match_file_extension;
use serde::{Deserialize, Serialize};
mod browser;
mod save_content;
mod thread_pool;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WebConfigData {
    pub websites: Vec<WebConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WebConfig {
    pub id: String,
    pub name: String,
    pub save_file: String,
    pub scraping: String,
    pub scraping_target: String,
    pub urls: Vec<String>,
}

//TODO
// add the choice of what to scrap in te config file directly

pub fn run_scrapper() {
    log::info_log("Getting the config file content...".to_string());
    let websites = utils::get_config_file_content();
    inquire::set_global_render_config(utils::get_render_config());
    let num_logical_cores: usize = num_cpus::get();
    let max_threads: usize = num_logical_cores * 2;
    scraper(websites, max_threads);
}

// fn scraper_option(websites: Vec<WebConfig>, max_threads: usize) {
//     match option {
//         "html-tag" => html_scraper(websites, max_threads),
//         "css-class" => css_scraper(websites, max_threads),
//         "id" => id_scraper(websites, max_threads),
//         _ => {
//             panic!()
//         }
//     }
// }

fn scraper(websites: Vec<WebConfig>, max_threads: usize) {
    log::info_log("Start scraping process...".to_string());
    let pool: thread_pool::MyThreadPool = thread_pool::MyThreadPool::new(max_threads);
    for website in websites.clone() {
        pool.queue_work(Box::new(move || scrap_website(&website)));
    }
}

fn scrap_website(website: &WebConfig) {
    let start = Instant::now();
    println!("Thread started for id: {}", website.id);
    let _html_option = "html-tag".to_string();
    for url in &website.urls {
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        match website.scraping.as_str() {
            "html-tag" => {
                parse_html_tag_and_save_content(
                    &url_clone,
                    save_file_clone,
                    &website.scraping_target,
                );
            }
            "css-class" => parse_css_class_and_save_content(
                &url_clone,
                save_file_clone,
                &website.scraping_target,
            ),
            _ => {}
        }
    }
    let duration = start.elapsed().as_secs_f32();
    println!(
        "Thread finished for id: {} in {:?} secondes",
        website.id, duration
    );
}

fn parse_html_tag_and_save_content(url: &str, save_file: String, tag_selector: &str) {
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_html_tag_content(html_from_browser.unwrap(), tag_selector.to_string());
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    let file_extension = utils::split_file_extension(&save_file);
    match_file_extension(parser, &save_file, &file_extension, tag_selector);
}

// parse the given html document
fn parse_html_tag_content(data: String, tag_selector: String) -> Vec<String> {
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

fn parse_css_class_and_save_content(url: &str, save_file: String, class_selector: &str) {
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_css_class_content(html_from_browser.unwrap(), class_selector);
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    let file_extension = utils::split_file_extension(&save_file);
    let class_selector_clone = class_selector;
    match_file_extension(parser, &save_file, &file_extension, &class_selector_clone);
}

fn parse_css_class_content(data: String, class_selector: &str) -> Vec<String> {
    let dom = tl::parse(&data, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let elements = dom.get_elements_by_class_name(&class_selector);
    let mut nodes = Vec::new();
    for element in elements {
        let node = element.get(parser).unwrap();
        nodes.push(node.inner_text(parser).to_string());
    }
    nodes
}

fn id_scraper(websites: Vec<WebConfig>, max_threads: usize) {
    let id = utils::prompt_message(
        "Which id do you want to scrap ?".to_string(),
        "Error getting the user input".to_string(),
    );
    log::info_log("Start scraping process...".to_string());
    let pool: thread_pool::MyThreadPool = thread_pool::MyThreadPool::new(max_threads);
    for website in websites.clone() {
        let id_clone = id.clone();
        pool.queue_work(Box::new(move || {
            download_website_by_id(&website, &id_clone)
        }));
    }
}

fn download_website_by_id(website: &WebConfig, id: &str) {
    let start = Instant::now();
    println!("Thread started for id: {}", website.id);
    for url in &website.urls {
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        parse_id_and_save_content(&url_clone, save_file_clone, id.to_string());
    }
    let duration = start.elapsed().as_secs_f32();
    println!(
        "Thread finished for id: {} in {:?} secondes",
        website.id, duration
    );
}

fn parse_id_and_save_content(url: &str, save_file: String, id: String) {
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_id_content(html_from_browser.unwrap(), id.clone());
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    let file_extension = utils::split_file_extension(&save_file);
    let id_clone = id.clone();
    match_file_extension(parser, &save_file, &file_extension, &id_clone);
}

fn parse_id_content(data: String, id: String) -> Vec<String> {
    let dom = tl::parse(&data, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let elements = dom.get_element_by_id(id.as_bytes());
    let mut nodes = Vec::new();
    if let Some(element) = elements {
        let node = element.get(parser).unwrap();
        nodes.push(node.inner_text(parser).to_string());
    }
    nodes
}
