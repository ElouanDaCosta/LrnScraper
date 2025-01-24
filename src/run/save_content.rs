use std::{error::Error, fs::OpenOptions, io::Write};

use csv::Writer;

use crate::log;

pub fn save_html_content(data: Vec<String>, filename: &str) {
    let filename_path = "data/".to_string() + filename;
    for i in data {
        let line_break = i + "\n";
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(filename_path.clone())
            .expect("Unable to open file");
        f.write_all(line_break.as_bytes())
            .expect("Unable to write data");
    }
}

pub fn save_in_csv(
    data: Vec<String>,
    filename: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let filename_path = "data/".to_string() + filename;
    let mut wtr = Writer::from_path(filename_path)?;
    wtr.write_record(&[selector])?;
    for i in data {
        wtr.write_record([i])?;
        wtr.flush()?;
    }
    Ok(())
}

pub fn match_file_extension(data: Vec<String>, filename: &str, extension: &str, selector: &str) {
    match extension {
        "txt" => save_html_content(data, filename),
        "csv" => {
            let error = save_in_csv(data, filename, selector);
            if let Err(e) = &error {
                let err = e.to_string();
                log::error_log_with_code("failed save in csv file".to_string(), err);
            }
        }
        _ => {
            panic!("Please refer txt or csv file.")
        }
    }
}
