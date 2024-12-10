use reqwest;
use scraper::{Html, Selector};
use tokio;

struct Product {
    url: Option<String>,
    image: Option<String>,
    title: Option<String>,
    price: Option<String>,
}

fn main() {
    let url = "https://scrapeme.live/shop/".to_string();
    scraper(url);
}

fn scraper(url: String) -> Vec<Product> {
    let response = reqwest::blocking::get(url).unwrap();

    let html_content: String = response.text().unwrap();

    let document = scraper::Html::parse_document(&html_content);

    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);
    let mut products: Vec<Product> = Vec::new();
    for html_product in html_products {
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let title = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        let product = Product {
            url,
            image,
            title,
            price,
        };
        products.push(product);
    }
    products
}
