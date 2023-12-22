use anyhow::Context;
use scraper::{Html, Selector};

struct ScrapedData {
    pub title: String,
    pub link: String,
}
pub async fn download_page_scraper(url: &str) -> anyhow::Result<Vec<ScrapedData>> {
    let body = reqwest::get(url).await?.text().await?;
    // Parse the HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".sbutton").unwrap();

    // Initialize an empty vector to store the scraped data
    let mut scraped_data = Vec::new();
    // Useing scraper to extract the link and title
    for element in document.select(&selector) {
        let a_tag = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap();
        let link = a_tag
            .value()
            .attr("href")
            .with_context(|| "Link not found")?;
        let song_type = a_tag.text().collect::<String>();

        let mut title = element
            .select(&Selector::parse(".bottom").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .with_context(|| "Couldn't parse the title")?;
        if song_type.contains("Full version") {
            title.push_str(" - (Full version)");
        }

        scraped_data.push(ScrapedData {
            title,
            link: link.to_string(),
        });
    }

    Ok(scraped_data)
}
