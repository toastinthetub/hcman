use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub name: String,
    pub regular_price: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub images: Vec<Image>,
    pub stock_quantity: Option<u32>,
    pub status: String,
    pub sku: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub src: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductRecord {
    pub title: String,
    pub description: String,
    pub category: String,
    pub price: f32,
    pub sku: String,
    pub status: String,
    pub images: String,
}
