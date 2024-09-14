use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
pub struct VendooProduct {
    #[serde(rename = "Title")]
    pub title: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "Category")]
    pub category: Option<String>,

    #[serde(rename = "Price")]
    pub price: Option<String>,

    #[serde(rename = "SKU")]
    pub sku: Option<String>,

    #[serde(rename = "Quantity")]
    pub quantity: Option<String>,

    #[serde(rename = "Condition")]
    pub condition: Option<String>,

    #[serde(rename = "Brand")]
    pub brand: Option<String>,

    #[serde(rename = "Images")]
    pub images: Option<String>,
}

pub fn read_vendoo_csv(file_path: &str) -> Result<Vec<VendooProduct>, Box<dyn Error>> {
    let file = File::open(Path::new(file_path))?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    let mut products = Vec::new();
    
    for result in rdr.deserialize() {
        match result {
            Ok(product) => products.push(product),
            Err(e) => eprintln!("Error parsing record: {:?}", e), // Print any error but continue processing
        }
    }

    Ok(products)
}

pub fn print_vendoo_product(product: &VendooProduct) {
    println!("Title: {}", product.title.as_deref().unwrap_or("N/A"));
    println!("Description: {}", product.description.as_deref().unwrap_or("N/A"));
    println!("Category: {}", product.category.as_deref().unwrap_or("N/A"));
    println!("Price: {}", product.price.as_deref().unwrap_or("N/A"));
    println!("SKU: {}", product.sku.as_deref().unwrap_or("N/A"));
    println!("Quantity: {}", product.quantity.as_deref().unwrap_or("N/A"));
    println!("Condition: {}", product.condition.as_deref().unwrap_or("N/A"));
    println!("Brand: {}", product.brand.as_deref().unwrap_or("N/A"));
    println!("Images: {}", product.images.as_deref().unwrap_or("N/A"));
    println!("-----------------------------------------");
}

pub fn print_all_vendoo_products(products: &[VendooProduct]) {
    for product in products {
        print_vendoo_product(product);
    }
}
