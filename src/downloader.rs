use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
    spawn,
    task::JoinSet,
};

use crate::scrapers::download_page::download_page_scraper;

pub async fn download_songs(url: &str) -> anyhow::Result<()> {
    let _ = fs::create_dir("downloads").await;

    let scraped_data = download_page_scraper(url).await?;
    let mut tasks = JoinSet::new();
    for element in scraped_data {
        let task =
            spawn(async move { download_song(&element.title, &element.link).await.unwrap() });
        tasks.spawn(task);
        let _ = tasks.join_next().await.unwrap();
    }

    Ok(())
}

pub async fn download_song(title: &str, link: &str) -> anyhow::Result<()> {
    tracing::info!("Started downloading: {}", title);

    let song_data = reqwest::get(link).await?.bytes().await?;
    let file_path = format!("downloads/{}.mp3", title);
    let file = File::create(&file_path).await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&song_data).await?;

    tracing::info!("Downloaded: {}", title);
    Ok(())
}
