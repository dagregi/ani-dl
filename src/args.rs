use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Scrape and download anime songs from animethemes.moe
pub struct Arguments {
    /// download all the themes
    #[arg(long)]
    pub all: bool,
    /// Search for a theme
    #[arg(long, value_name = "THEME")]
    pub search: Option<String>,
}
