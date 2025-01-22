use headless_chrome::{Browser, LaunchOptions};

pub fn browse_website(input: &str) -> Result<String, anyhow::Error> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(true)
            .build()
            .expect("Could not find chrome-executable"),
    );

    let browser = match browser {
        Ok(browser) => browser,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(anyhow::anyhow!("Failed to launch browser"));
        }
    };
    let tab = match browser.new_tab() {
        Ok(tab) => tab,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(anyhow::anyhow!("Failed to create new tab"));
        }
    };

    let website = tab.navigate_to(input);
    return website.unwrap().get_content();
}
