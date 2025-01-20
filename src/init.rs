use crate::{template, utils};
use std::{fs::File, io::Write};

pub fn init_config() {
    utils::rustyspider_ascii_art();
    println!("Initialize RustySpider configuration file...");
    let mut f = File::create("rustyspider_config.json").expect("Unable to create file");
    f.write_all(&template::config_file_template().as_bytes())
        .expect("Unable to write data");
    println!("Configuration file initialize successfully!");
}
