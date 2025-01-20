use std::process::exit;

pub fn rustyspider_usage() {
    let usage = r"
Usage: rustyspider command [options]

RustySpider's web scraper cli.

Commands:
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
