use std::path::PathBuf;

use anyhow::Context;
use scraper::{Html, Selector};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}

struct ScrapedData {
    title: String,
    link: String,
}
async fn scrape_download_page(url: &str) -> anyhow::Result<Vec<ScrapedData>> {
    let body = reqwest::get(url).await?.text().await?;
    // Parse the HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".sbutton").unwrap();

    // Initialize an empty vector to store the scraped data
    let mut scraped_data = Vec::new();
    // Useing scraper to extract the link and title
    for element in document.select(&selector) {
        let a_tag = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap();
        let link = a_tag
            .value()
            .attr("href")
            .with_context(|| "Link not found")?;
        let song_type = a_tag.text().collect::<String>();

        let mut title = element
            .select(&Selector::parse(".bottom").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .with_context(|| "Couldn't parse the title")?;
        if song_type.contains("Full version") {
            title.push_str(" - (Full version)");
        }

        scraped_data.push(ScrapedData {
            title,
            link: link.to_string(),
        });
    }

    Ok(scraped_data)
}

async fn download_songs(url: &str) -> anyhow::Result<()> {
    // Create a new directory to save the downloaded songs
    let _ = fs::create_dir("anime-songs").await;

    let scraped_data = scrape_download_page(url).await?;
    // Iterate over the song links and download each song
    for element in scraped_data {
        let title = element.title;
        let link = element.link;

        // Download the song
        tracing::info!("Downloading: {}", title);
        let song_data = reqwest::get(link).await?.bytes().await?;
        // Save the song to a file
        let file_path = format!("anime-songs/{}.mp3", title);
        let mut file = File::create(&file_path).await?;
        file.write_all(&song_data).await?;
        tracing::info!("Downloaded: {}", title);
    }

    tracing::info!("All songs downloaded!");
    Ok(())
}
