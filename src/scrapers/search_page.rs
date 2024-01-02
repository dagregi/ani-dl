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
    anime: Anime,
    song: Song,
    animethemeentries: Vec<AnimeThemeEntry>,
}

#[derive(Debug, Deserialize)]
struct Anime {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Song {
    title: String,
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

#[derive(Debug)]
pub struct AnimeThemeData {
    pub anime_name: String,
    pub song_title: String,
    pub basename: String,
}
pub async fn search_page_scraper(query: &str) -> anyhow::Result<HashMap<String, AnimeThemeData>> {
    let url = format!("https://api.animethemes.moe/animetheme?q={}&filter%5Bhas%5D=song&include=song.artists,anime.images,animethemeentries.videos", query);

    println!("Searching for: {}\n", query);
    let response = reqwest::get(url).await?.text().await?;
    let api_data: ApiData = serde_json::from_str(&response)?;
    let mut results = HashMap::with_capacity(api_data.animethemes.len());

    api_data.animethemes.into_iter().for_each(|anime_theme| {
        anime_theme
            .animethemeentries
            .into_iter()
            .for_each(|anime_theme_entry| {
                anime_theme_entry.videos.into_iter().for_each(|video| {
                    results.entry(video.link).or_insert(AnimeThemeData {
                        anime_name: anime_theme.anime.name.to_string(),
                        song_title: anime_theme.song.title.to_string(),
                        basename: video.basename,
                    });
                });
            });
    });

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

    results.values().enumerate().for_each(|(i, v)| {
        println!(
            "\x1B[1;37m{}\x1B[0m. \x1B[1;36m{}\x1B[0m \x1B[2;37m({})\x1B[0m",
            i, v.song_title, v.anime_name
        );
    });
    println!("\x1B[1;37m99. Download all songs");
    println!("\x1B[1;37mq. Quit");

    print!("\x1B[1;32mYour choice >\x1B[0m ");
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let choices: Vec<_> = buffer.split_whitespace().collect();

    if choices.contains(&"q") {
        return Ok(());
    }
    for c in choices.iter() {
        if let Ok(i) = c.parse::<usize>() {
            if i < results.len() {
                download_songs(results.keys().nth(i).unwrap()).await?;
            } else if i == 99 {
                for link in results.keys() {
                    download_songs(link).await?;
                }
            }
        }
    }

    Ok(())
}
