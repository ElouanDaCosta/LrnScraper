use std::process::exit;

use colored::Colorize;

pub fn rustyspider_usage() {
    let usage = r"
Usage: rustyspider command [options]

RustySpider's web scraper cli.

Commands:
    run             Run the scraping process
    init            Init the config file of RustySpider
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
