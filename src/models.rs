use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub src: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

