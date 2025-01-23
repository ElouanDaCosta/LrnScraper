use std::fs;

pub fn save_html_content(data: Vec<String>, filename: &str) {
    for i in data {
        fs::write(filename, i).expect("Unable to write file");
    }
}
