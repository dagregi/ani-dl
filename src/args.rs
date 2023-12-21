use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Scrape and download anime songs from mp3anime.net
pub struct Arguments {
    /// Scrape all of mp3anime.net (may take some time)
    #[arg(long)]
    pub all: bool,
    /// Search for a song and download it
    #[arg(long, value_name = "SONG")]
    pub search: Option<String>,
}
