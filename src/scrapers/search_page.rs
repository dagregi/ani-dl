use anyhow::Context;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::io::Write;

use crate::downloader::download_songs;

pub async fn search_page_scraper(query: &str) -> anyhow::Result<HashMap<String, String>> {
    let mut results = HashMap::new();

    tracing::info!("Searching for: {}", query);
    let body = reqwest::get(format!("https://mp3anime.net/songs/?q={}", query))
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("table td a").unwrap();
    for element in document.select(&selector) {
        let title = element.text().collect::<String>();
        let link = element
            .value()
            .attr("href")
            .with_context(|| "Link not found")?;
        let full_link = format!("https://mp3anime.net{}", link);
        if full_link.contains("songs") {
            results.insert(full_link, title);
        }
    }

    Ok(results)
}

pub async fn search_results(query: &str) -> anyhow::Result<()> {
    let results = search_page_scraper(query).await?;

    results.values().enumerate().for_each(|(i, v)| println!("{}. {}", i, v));
    println!("99. Download all songs");

    print!("Your choice > ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();

    match buffer.parse::<usize>() {
        Ok(i) => {
            if i < results.len() {
                download_songs(results.keys().nth(i).unwrap()).await?;
            } else if i == 99 {
                for link in results.keys() {
                    download_songs(link).await?;
                }
            }
        }
        Err(_) => eprintln!("Errored out")
    }
    Ok(())
}
