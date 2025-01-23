use std::fs;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;
extern crate num_cpus;

use crate::log;
use save_content::save_html_content;
use serde::{Deserialize, Serialize};
mod browser;
mod save_content;

use std::{collections::VecDeque, thread::JoinHandle};

pub struct MyThreadPool {
    work_queue: Arc<Mutex<VecDeque<Box<dyn Send + FnOnce() -> ()>>>>, // A.
    join_handles: Vec<Option<JoinHandle<()>>>,                        // B.

    work_item_count: Arc<Mutex<i32>>, // C. // -1 implies that threads should wind down.
    signal: Arc<Condvar>,             // D.
}

impl Drop for MyThreadPool {
    fn drop(&mut self) {
        {
            // A.
            let mut g = self.work_item_count.lock().unwrap();
            (*g) = -1; // B.
            self.signal.notify_all();
        }

        for h in &mut self.join_handles {
            // C.
            let x: JoinHandle<()> = std::mem::replace(h, None).unwrap();
            _ = x.join();
        }
    }
}

impl MyThreadPool {
    pub fn new(n: usize) -> Self {
        let workq = Arc::new(Mutex::new(VecDeque::new()));
        let itemcount = Arc::new(Mutex::new(0_i32));
        let signal = Arc::new(Condvar::new());

        let mut pool: MyThreadPool = MyThreadPool {
            // A.
            work_queue: workq.clone(),
            join_handles: Vec::with_capacity(n),
            work_item_count: itemcount.clone(),
            signal: signal.clone(),
        };

        for _idx in 0..n {
            let wq = workq.clone();
            let itmcnt = itemcount.clone();
            let sig = signal.clone();

            let h: JoinHandle<()> = std::thread::spawn(move || {
                // B.
                loop {
                    let shouldbreak: bool = {
                        let mut g = itmcnt.lock().unwrap();
                        while *g == 0 {
                            g = sig.wait(g).unwrap(); // C.
                            if (*g) == 0 {
                                println!("{:?} Spurious Wakeup.", std::thread::current().id());
                            }
                        }

                        if (*g) > 0 {
                            // E.
                            (*g) -= 1;
                        }

                        // F.
                        (*g) == -1
                    };

                    if shouldbreak {
                        // G.
                        break;
                    }

                    let workfnopt = {
                        let mut g = wq.lock().unwrap();
                        g.pop_back()
                    };

                    if let Some(workfn) = workfnopt {
                        workfn(); // H.
                    } else {
                        println!(
                            "Worker {:?} SOMETHING IS WRONG!",
                            std::thread::current().id()
                        );
                    }
                }

                // Wind-down loop
                loop {
                    // I.
                    let workfnopt = {
                        let mut g = wq.lock().unwrap();
                        g.pop_back()
                    };

                    if let Some(workfn) = workfnopt {
                        workfn();
                    } else {
                        break;
                    }
                }
            });

            pool.join_handles.push(Some(h)); // J.
        }

        pool
    }

    pub fn queue_work(&self, work: Box<dyn Send + FnOnce() -> ()>) {
        // K.
        let mut g = self.work_item_count.lock().unwrap();
        assert!((*g) >= 0);
        (*g) += 1;
        let mut guard = self.work_queue.lock().unwrap();
        guard.push_front(work);
        self.signal.notify_one(); // L.
    }
}

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
    let num_logical_cores = num_cpus::get();
    let max_threads = num_logical_cores * 2;
    let pool: MyThreadPool = MyThreadPool::new(2);
    for website in websites.clone() {
        pool.queue_work(Box::new(move || download_website(&website)));
    }
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
    let start = Instant::now();
    let num_logical_cores = num_cpus::get();
    let max_threads = num_logical_cores * 2;
    let pool: MyThreadPool = MyThreadPool::new(max_threads - 2);
    for url in &website.urls {
        // let response = reqwest::blocking::get(url);
        // let html_from_request = response.unwrap().text().unwrap();
        let id_clone = website.id.clone();
        let save_file_clone = website.save_file.clone();
        let url_clone = url.clone();
        pool.queue_work(Box::new(move || {
            sub_thread_url(&url_clone, id_clone, save_file_clone)
        }));
    }
    let duration = start.elapsed().as_secs_f32();
    println!(
        "Thread finished for id: {} in {:?} secondes",
        website.id, duration
    );
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
            "Error getting the content for url:".to_string(),
            url.to_string(),
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
