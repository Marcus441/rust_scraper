use reqwest::Client;

struct Product {
    prod_url: Option<String>,
    prod_image: Option<String>,
    title: Option<String>,
    price: Option<String>,
}

#[tokio::main]
async fn main() {
    let url = "https://scrapeme.live/shop/".to_string();
    let products: Vec<Product> = scraper(url).await;

    for product in products {
        println!(
            "name = {:?}, url= {:?}, image= {:?}, price= {:?}",
            product.title, product.prod_url, product.prod_image, product.price
        );
    }
}

async fn scraper(url: String) -> Vec<Product> {
    let client = Client::new();

    let response = client.get(url).send().await.unwrap();

    let html_content: String = response.text().await.unwrap();

    let document = scraper::Html::parse_document(&html_content);

    let html_product_selector = scraper::Selector::parse("li.product").unwrap();

    let html_products = document.select(&html_product_selector);
    let mut products: Vec<Product> = Vec::new();

    for html_product in html_products {
        let prod_url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let prod_image = html_product
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

        products.push(Product {
            prod_url,
            prod_image,
            title,
            price,
        });
    }
    products
}
