use reqwest::Client;
use csv::ReaderBuilder;
use std::error::Error;
use crate::models::{Product, ProductRecord, Category, Image};

pub async fn fetch_all_products(api_url: &str, consumer_key: &str, consumer_secret: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let url = format!("{}/products", api_url);
    let client = Client::new();
    let response = client
        .get(&url)
        .basic_auth(consumer_key, Some(consumer_secret))
        .send()
        .await?;
        
    if response.status().is_success() {
        let products: Vec<Product> = response.json().await?;
        Ok(products)
    } else {
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}

// this reads a whole csv file and just posts it all. DO NOT TOUCH
pub async fn post_products_from_csv(csv_file: &str, api_url: &str, consumer_key: &str, consumer_secret: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_path(csv_file)?;

    for result in rdr.deserialize() {
        let record: ProductRecord = result?;
        if record.status.to_lowercase() == "active" {
            let product_data = Product {
                name: record.title,
                regular_price: record.price.to_string(),
                description: record.description,
                categories: vec![Category { name: record.category }],
                images: vec![Image { src: record.images }],
                stock_quantity: Some(1),
                status: "publish".to_string(),
                sku: record.sku,
            };
            post_product_to_wc(product_data, api_url, consumer_key, consumer_secret).await?;
        }
    }

    Ok(())
}

// posts a single product to woocommerce
async fn post_product_to_wc(product: Product, api_url: &str, consumer_key: &str, consumer_secret: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/products", api_url);
    let client = Client::new();
    let response = client
        .post(&url)
        .basic_auth(consumer_key, Some(consumer_secret))
        .json(&product)
        .send()
        .await?;

    if response.status().is_success() {
        println!("product '{}' posted", product.name);
    } else {
        eprintln!("failed to post as a result of this error: {}", response.status());
    }

    Ok(())
}
