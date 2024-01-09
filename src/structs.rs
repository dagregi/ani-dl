use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiData {
    pub animethemes: Vec<AnimeTheme>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeTheme {
    pub anime: Anime,
    pub song: Song,
    #[serde(rename = "type")]
    pub type_var: String,
    pub animethemeentries: Vec<AnimeThemeEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Song {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct AnimeThemeEntry {
    pub videos: Vec<Video>,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub basename: String,
    pub link: String,
}

#[derive(Debug, Clone)]
pub struct AnimeThemeMetaData {
    pub anime_name: String,
    pub song_title: String,
    pub song_type: String,
    pub basename: String,
}
impl AnimeThemeMetaData {
    pub fn new() -> Self {
        AnimeThemeMetaData {
            anime_name: String::new(),
            song_title: String::new(),
            song_type: String::new(),
            basename: String::new(),
        }
    }

    fn insert_metadata(
        &self,
        anime_theme: &AnimeTheme,
        _anime_theme_entry: &AnimeThemeEntry,
        video: &Video,
        results: &mut HashMap<String, AnimeThemeMetaData>,
    ) {
        results
            .entry(video.link.to_owned())
            .or_insert_with(|| AnimeThemeMetaData {
                anime_name: anime_theme.anime.name.to_owned(),
                song_title: anime_theme.song.title.to_owned(),
                song_type: anime_theme.type_var.to_owned(),
                basename: video.basename.to_owned(),
            });
    }

    pub fn process_animethemes(
        &self,
        animethemes: &[AnimeTheme],
        song_type: &str,
        results: &mut HashMap<String, AnimeThemeMetaData>,
    ) {
        let filtered_animethemes = animethemes.iter().filter(|anime_theme| {
            anime_theme.type_var == song_type.to_uppercase() || song_type.is_empty()
        });

        filtered_animethemes.for_each(|anime_theme| {
            anime_theme
                .animethemeentries
                .iter()
                .for_each(|anime_theme_entry| {
                    anime_theme_entry.videos.iter().for_each(|video| {
                        self.insert_metadata(anime_theme, anime_theme_entry, video, results);
                    });
                });
        });
    }
}
