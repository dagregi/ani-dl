use anyhow::Context;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;

use crate::downloader::download_songs;

#[derive(Debug, Deserialize)]
struct ApiData {
    animethemes: Vec<AnimeTheme>,
}

#[derive(Debug, Deserialize)]
struct AnimeTheme {
    animethemeentries: Vec<AnimeThemeEntry>,
}

#[derive(Debug, Deserialize)]
struct AnimeThemeEntry {
    videos: Vec<Video>,
}

#[derive(Debug, Deserialize)]
struct Video {
    basename: String,
    link: String,
}

pub async fn search_page_scraper(query: &str) -> anyhow::Result<HashMap<String, String>> {
    let mut results = HashMap::new();
    let url = format!("https://api.animethemes.moe/animetheme?q={}&filter%5Bhas%5D=song&include=animethemeentries.videos", query);

    println!("Searching for: {}", query);
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let api_data: ApiData = serde_json::from_str(&text).expect("Failed to parse JSON");

    for anime_theme in api_data.animethemes {
        for anime_theme_entry in anime_theme.animethemeentries {
            for video in anime_theme_entry.videos {
                results.insert(video.link, video.basename);
            }
        }
    }

    Ok(results)
}

pub async fn search_results(query: &str, is_save_all: bool) -> anyhow::Result<()> {
    let results = search_page_scraper(query).await?;
    if results.is_empty() {
        eprintln!("No results found");
        return Ok(());
    }
    if is_save_all {
        for link in results.keys() {
            download_songs(link).await?;
        }
        return Ok(());
    }

    results
        .values()
        .enumerate()
        .for_each(|(i, v)| println!("{}. {}", i, v));
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
        Err(_) => eprintln!("Errored out"),
    }
    Ok(())
}
