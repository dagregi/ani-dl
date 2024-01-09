use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::collections::HashMap;
use std::fmt::Write;
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
    spawn,
    task::JoinSet,
};

use crate::structs::AnimeThemeMetaData;

pub async fn download_songs(themes: HashMap<String, AnimeThemeMetaData>) -> anyhow::Result<()> {
    let mut tasks = JoinSet::new();
    for (link, metadata) in themes {
        let task = spawn(async move { download_song(&metadata, &link).await.unwrap() });
        tasks.spawn(task);
        let _ = tasks.join_next().await.unwrap();
    }

    Ok(())
}

pub async fn download_song(metadata: &AnimeThemeMetaData, link: &str) -> anyhow::Result<()> {
    let output_dir = "downloads";
    let _ = fs::create_dir(output_dir).await;
    let mut song_data = reqwest::get(link).await?;

    let mut downloaded = 0;
    let total_size = song_data.content_length().unwrap();

    let title = &metadata.song_title;
    let basename = &metadata.basename;

    println!("Downloading \x1B[1;36m{}\x1B[0m", title);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));

    let file_path = format!("{}/{} - {}", output_dir, title, basename);
    let file = File::create(&file_path).await?;
    let mut writer = BufWriter::with_capacity(8192, file);
    while let Some(chunk) = song_data.chunk().await? {
        downloaded += chunk.len() as u64;
        writer.write_all(&chunk).await?;
        pb.set_position(downloaded);
    }
    writer.flush().await?;
    pb.finish();

    println!("Saved to \x1B[1;36m{}\x1B[0m", file_path);

    Ok(())
}
