extern crate num_cpus;

use std::time::Instant;

use crate::log;
use crate::utils;
use save_content::save_html_content;
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
    pub urls: Vec<String>,
}

pub fn run_scrapper() {
    log::info_log("Getting the config file content...".to_string());
    let websites = utils::get_config_file_content();
    inquire::set_global_render_config(utils::get_render_config());
    let select_option = utils::get_select_option(
        "Select what do you want to scrap:".to_string(),
        utils::get_scraper_option(),
    )
    .unwrap();
    let num_logical_cores: usize = num_cpus::get();
    let max_threads: usize = num_logical_cores * 2;
    scraper_option(&select_option, websites, max_threads);
    log::info_log("Start scraping process...".to_string());
}

fn scraper_option(option: &str, websites: Vec<WebConfig>, max_threads: usize) {
    match option {
        "html-tag" => html_scraper(websites, max_threads),
        "css-class" => css_scraper(websites, max_threads),
        "id" => id_scraper(websites, max_threads),
        _ => {
            panic!()
        }
    }
}

fn html_scraper(websites: Vec<WebConfig>, max_threads: usize) {
    let html_tag = utils::prompt_message(
        "Which html-tag do you want to scrap ?".to_string(),
        "Error getting the user input".to_string(),
    );
    let pool: thread_pool::MyThreadPool = thread_pool::MyThreadPool::new(max_threads);
    for website in websites.clone() {
        let html_tag_clone = html_tag.clone();
        pool.queue_work(Box::new(move || {
            download_website_by_html(&website, &html_tag_clone)
        }));
    }
}

fn download_website_by_html(website: &WebConfig, html_tag: &str) {
    let start = Instant::now();
    println!("Thread started for id: {}", website.id);
    for url in &website.urls {
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        parse_html_tag_and_save_content(&url_clone, save_file_clone, html_tag.to_string());
    }
    let duration = start.elapsed().as_secs_f32();
    println!(
        "Thread finished for id: {} in {:?} secondes",
        website.id, duration
    );
}

fn parse_html_tag_and_save_content(url: &str, save_file: String, tag_selector: String) {
    let html_from_browser = browser::browse_website(&url);
    if html_from_browser.is_err() {
        log::error_log(html_from_browser.as_ref().unwrap_err().to_string());
    }
    let parser = parse_html_tag_content(html_from_browser.unwrap(), tag_selector);
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    save_html_content(parser, &save_file);
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

fn css_scraper(websites: Vec<WebConfig>, max_threads: usize) {
    let html_tag = utils::prompt_message(
        "Which css-class do you want to scrap ?".to_string(),
        "Error getting the user input".to_string(),
    );
    let pool: thread_pool::MyThreadPool = thread_pool::MyThreadPool::new(max_threads);
    for website in websites.clone() {
        let html_tag_clone = html_tag.clone();
        pool.queue_work(Box::new(move || {
            download_website_by_css(&website, &html_tag_clone)
        }));
    }
}

fn download_website_by_css(website: &WebConfig, css_class: &str) {
    let start = Instant::now();
    println!("Thread started for id: {}", website.id);
    for url in &website.urls {
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        parse_css_class_and_save_content(&url_clone, save_file_clone, css_class.to_string());
    }
    let duration = start.elapsed().as_secs_f32();
    println!(
        "Thread finished for id: {} in {:?} secondes",
        website.id, duration
    );
}

fn parse_css_class_and_save_content(url: &str, save_file: String, class_selector: String) {
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
    save_html_content(parser, &save_file);
}

fn parse_css_class_content(data: String, class_selector: String) -> Vec<String> {
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
    let parser = parse_id_content(html_from_browser.unwrap(), id);
    if parser.is_empty() {
        log::error_log_with_code(
            "Error getting the content for url:".to_string(),
            url.to_string(),
        );
    }
    save_html_content(parser, &save_file);
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
