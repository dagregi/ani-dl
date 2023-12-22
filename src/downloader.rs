use anyhow::Context;
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
    spawn,
};

use crate::scrapers::download_page::download_page_scraper;

pub async fn download_songs(url: &str) -> anyhow::Result<()> {
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

pub async fn download_song(title: &str, link: &str) -> anyhow::Result<()> {
    tracing::info!("Started downloading: {}", title);

    let song_data = reqwest::get(link).await?.bytes().await?;
    let file_path = format!("anime-songs/{}.mp3", title);
    let file = File::create(&file_path).await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&song_data).await?;

    tracing::info!("Downloaded: {}", title);
    Ok(())
}
