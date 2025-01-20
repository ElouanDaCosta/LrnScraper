use crate::template;
use std::{fs::File, io::Write};

pub fn init_config() {
    let mut f = File::create("rustyspider_config.json").expect("Unable to create file");
    f.write_all(&template::config_file_template().as_bytes())
        .expect("Unable to write data");
}
