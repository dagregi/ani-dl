mod args;
mod downloader;
pub mod scrapers;

use clap::Parser;
use scrapers::search_page::search_page_scraper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let cli_args = args::Arguments::parse();
    if let Some(search) = cli_args.search {
        search_page_scraper(&search).await?;
    }
    Ok(())
}
