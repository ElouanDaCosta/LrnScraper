use std::{fs, process::exit};

use colored::Colorize;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    InquireError, Select, Text,
};

use crate::run::{WebConfig, WebConfigData};

pub fn rustyspider_usage() {
    let usage = r"
Usage: rustyspider command [options]

RustySpider's web scraper cli.

Commands:
    run             Run the scraping process
    init            Init the config file of RustySpider
    clean           Clean the data folder from all files inside
    help            Show this help message

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of RustySpider
";

    println!("{}", usage);
}

pub fn command_usage(usage: &str) {
    println!("{}", usage);
    exit(0);
}

pub fn rustyspider_ascii_art() {
    let ascii = r"
__________                 __             _________        .__     .___              
\______   \ __ __  _______/  |_  ___.__. /   _____/______  |__|  __| _/ ____ _______ 
 |       _/|  |  \/  ___/\   __\<   |  | \_____  \ \____ \ |  | / __ |_/ __ \\_  __ \
 |    |   \|  |  /\___ \  |  |   \___  | /        \|  |_> >|  |/ /_/ |\  ___/ |  | \/
 |____|_  /|____//____  > |__|   / ____|/_______  /|   __/ |__|\____ | \___  >|__|   
        \/            \/         \/             \/ |__|             \/     \/                                                                             
  ";
    println!("{}", ascii.truecolor(255, 94, 0))
}

pub fn get_config_file_content() -> Vec<WebConfig> {
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

pub fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("?").with_fg(Color::LightMagenta);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::DarkMagenta);
    render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightMagenta);
    render_config.scroll_up_prefix = Styled::new("⇞").with_fg(Color::DarkMagenta);
    render_config.scroll_down_prefix = Styled::new("⇟").with_fg(Color::DarkMagenta);
    render_config.unselected_checkbox = Styled::new("☐").with_fg(Color::DarkMagenta);
    render_config.selected_option = Some(StyleSheet::new().with_fg(Color::DarkMagenta));
    render_config.text_input = StyleSheet::new().with_fg(Color::DarkMagenta);

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}

pub fn get_select_option(
    prompt: String,
    option: Vec<String>,
) -> std::result::Result<String, InquireError> {
    let ans: std::result::Result<String, InquireError> = Select::new(&prompt, option).prompt();

    return ans;
}

pub fn get_scraper_option() -> Vec<String> {
    let scraper_option: Vec<String> = vec![
        "html-tag".to_string(),
        "css-class".to_string(),
        "id".to_string(),
    ];
    scraper_option
}

pub fn prompt_message(message: String, error_message: String) -> String {
    inquire::set_global_render_config(get_render_config());
    let message = Text::new(&message).prompt().expect(&error_message);
    return message;
}
