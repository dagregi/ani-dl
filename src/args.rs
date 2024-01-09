use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Scrape and download anime songs from animethemes.moe
pub struct Arguments {
    /// Batch download themes
    #[arg(long)]
    pub all: bool,
    /// Search for a theme
    #[arg(long, value_name = "THEME")]
    pub search: Option<String>,
    /// Specify the type of the theme
    #[arg(short, long, value_name = "OP/ED")]
    pub type_: Option<String>,
}
