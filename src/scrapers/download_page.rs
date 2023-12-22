use anyhow::Context;
use scraper::{Html, Selector};

pub struct ScrapedData {
    pub title: String,
    pub link: String,
}
pub async fn download_page_scraper(url: &str) -> anyhow::Result<Vec<ScrapedData>> {
    let body = reqwest::get(url).await?.text().await?;
    // Parse the HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".sbutton").unwrap();
    // Extract the data and collect it
    let scraped_data = document
        .select(&selector)
        .map(|element| {
            let tag = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap();
            let link = tag
                .value()
                .attr("href")
                .with_context(|| "Link not found")?
                .to_string();
            let song_type = tag.text().collect::<String>();
            let title = element
                .select(&Selector::parse(".bottom").unwrap())
                .next()
                .map(|e| e.text().collect::<String>())
                .with_context(|| "Couldn't parse the title")?;
            let title = if song_type.contains("Full version") {
                format!("{} - (Full version)", title)
            } else {
                title
            };
            Ok(ScrapedData { title, link })
        })
        .collect::<anyhow::Result<Vec<ScrapedData>>>()?;

    Ok(scraped_data)
}
