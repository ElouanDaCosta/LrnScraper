use std::{fs::File, io::Write};

pub fn init_config() {
    let mut f = File::create("rustyspider_config.yaml").expect("Unable to create file");
    f.write_all(&"Hello RustySpiders !".as_bytes())
        .expect("Unable to write data");
}
