use std::collections::HashMap;

use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
    spawn,
    task::JoinSet,
};

use crate::scrapers::search_page::AnimeThemeData;

pub async fn download_songs(themes: HashMap<String, AnimeThemeData>) -> anyhow::Result<()> {
    let mut tasks = JoinSet::new();
    for (link, metadata) in themes {
        let task = spawn(async move { download_song(&metadata, &link).await.unwrap() });
        tasks.spawn(task);
        let _ = tasks.join_next().await.unwrap();
    }

    Ok(())
}

pub async fn download_song(metadata: &AnimeThemeData, link: &str) -> anyhow::Result<()> {
    let output_dir = "downloads";
    let _ = fs::create_dir(output_dir).await;

    let title = &metadata.song_title;
    let basename = &metadata.basename;

    println!("Downloading \x1B[1;36m{}\x1B[0m", title);
    let song_data = reqwest::get(link).await?.bytes().await?;
    let file_path = format!("{}/{} - {}", output_dir, title, basename);
    let file = File::create(&file_path).await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&song_data).await?;

    println!("Saved to \x1B[1;36m{}\x1B[0m", file_path);
    Ok(())
}
