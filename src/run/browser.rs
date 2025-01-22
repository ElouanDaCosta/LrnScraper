use headless_chrome::{Browser, LaunchOptions};

pub fn browse_website(input: &str) -> Result<String, anyhow::Error> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )
    .unwrap();
    let tab = browser.new_tab().unwrap();
    let website = tab.navigate_to(input);

    return website.unwrap().get_content();
}
