use anyhow::Context;
use scraper::{Html, Selector};

use crate::downloader::download_songs;

pub async fn search_page_scraper(query: &str) -> anyhow::Result<()> {
    tracing::info!("Searching for: {}", query);
    let body = reqwest::get(format!("https://mp3anime.net/songs/?q={}", query))
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("table td a").unwrap();
    // Iterate over the table rows and extract the link
    for element in document.select(&selector) {
        let link = element
            .value()
            .attr("href")
            .with_context(|| "Link not found")?;
        let full_link = format!("https://mp3anime.net{}", link);
        if full_link.contains("songs") {
            download_songs(&full_link).await?;
        }
    }

    Ok(())
}
