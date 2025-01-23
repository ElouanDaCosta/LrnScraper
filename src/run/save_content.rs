use std::{fs::OpenOptions, io::Write};

pub fn save_html_content(data: Vec<String>, filename: &str) {
    let filename_path = "data/".to_string() + filename;
    for i in data {
        let line_break = i + ".txt\n";
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(filename_path.clone())
            .expect("Unable to open file");
        f.write_all(line_break.as_bytes())
            .expect("Unable to write data");
    }
}
