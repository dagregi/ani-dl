mod args;

use anyhow::Context;
use clap::Parser;
use scraper::{Html, Selector};
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
    spawn
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let cli_args = args::Arguments::parse();
    if let Some(search) = cli_args.search {
        search_page_parser(&search).await?;
    }
    Ok(())
}

struct ScrapedData {
    title: String,
    link: String,
}
async fn download_page_scraper(url: &str) -> anyhow::Result<Vec<ScrapedData>> {
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
async fn search_page_parser(query: &str) -> anyhow::Result<()> {
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

async fn download_songs(url: &str) -> anyhow::Result<()> {
    // Create a new directory to save the downloaded songs
    let _ = fs::create_dir("anime-songs").await;

    let scraped_data = download_page_scraper(url).await?;
    let mut tasks = Vec::new();
    // Iterate over the song links and download each song
    for element in scraped_data {
        let title = element.title;
        let link = element.link;
        // Create a new task for each song
        let task = spawn(async move { download_song(&title, &link).await.unwrap() });
        tasks.push(task);
        // time::sleep(time::Duration::from_secs(2)).await;
    }
    // Wait for all the tasks to complete
    for task in tasks {
        task.await.with_context(|| "Failed to join task").unwrap();
    }
    Ok(())
}

async fn download_song(title: &str, link: &str) -> anyhow::Result<()> {
    tracing::info!("Started downloading: {}", title);
    let song_data = reqwest::get(link).await?.bytes().await?;
    let file_path = format!("anime-songs/{}.mp3", title);
    let file = File::create(&file_path).await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&song_data).await?;

    tracing::info!("Downloaded: {}", title);
    Ok(())
}
