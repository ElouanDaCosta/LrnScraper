# LrnScraper

A Web Scraper written in Rust.

## Usage

```bash
# Initialize a new configuration file
lrnscrap init
```

```bash
# Run the web scraper
lrnscrap run
```

```bash
# Clean the data folder
lrnscrap clean
```

## Configuration File

You can configure the web scraper by editing the `config.json` file.
You can add as many websites as you want. Each website must have an `id`, a `name`, a `save_file`, a `scraping`, a `scraping_target` and a list of `urls`.
You can scrap the website by `id` by `html-tag` or by `css-class`.

### Example

```json
{
  "websites": [
    {
      "id": "app",
      "name": "Apple website",
      "save_file": "Apple.csv",
      "scraping": "id",
      "scraping_target": "globalnav-menubutton-link-search",
      "urls": [
        "https://www.apple.com/fr/"
      ]
    },
    {
      "id": "wik",
      "name": "Wikipedia website",
      "save_file": "Wikipedia.txt",
      "scraping": "html-tag",
      "scraping_target": "h2",
      "urls": [
        "https://fr.wikipedia.org/wiki/Wikipédia:Accueil_principal"
      ]
    }
  ]
}
```

The given extension of the `save_file` will determine the format of the output file. The supported formats are `csv` and `txt`.
