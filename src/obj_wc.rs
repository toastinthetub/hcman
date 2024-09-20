use std::{fmt::format, iter::Product};

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ObjWooCommerce {
    pub db_path: Option<String>,
    pub products: Option<Vec<WooCommerceProduct>>,
    base_api: String,
    pub skey: String, // WC secret key
    pub ckey: String, // WC consumer key
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WooCommerceProduct {
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

impl ObjWooCommerce {
    pub fn new_with_auth(base_api: String, ckey: String, skey: String) -> Self {
        let db_path: Option<String> = None;
        let products: Option<Vec<WooCommerceProduct>> = None;
        Self {
            db_path,
            products,
            base_api,
            skey,
            ckey,
        }
    }
    // Fetches all products and populates self.products
    pub async fn fetch_populate_products(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/products", self.base_api.trim_end_matches('/')); // trim trailing slashes
        let client = Client::new();
        let response = client
            .get(&url)
            .basic_auth(&self.ckey, Some(&self.skey))
            .send()
            .await?;

        if response.status().is_success() {
            let products: Vec<WooCommerceProduct> = response.json().await?;
            self.products = Some(products);
            Ok(())
        } else {
            let status = response.status();
            let error_msg = format!("Failed to fetch products: {}", status);
            Err(error_msg.into())
        }
    }

    // Just fetches and returns the products without populating the object
    pub async fn fetch_products_raw(
        &self,
    ) -> Result<Vec<WooCommerceProduct>, Box<dyn std::error::Error>> {
        let url = format!("{}/products", self.base_api.trim_end_matches('/')); // trim trailing slashes
        let client = Client::new();
        let response = client
            .get(&url)
            .basic_auth(&self.ckey, Some(&self.skey))
            .send()
            .await?;

        if response.status().is_success() {
            let products: Vec<WooCommerceProduct> = response.json().await?;
            Ok(products)
        } else {
            let status = response.status();
            let error_msg = format!("Failed to fetch products: {}", status);
            Err(error_msg.into())
        }
    }
}

impl WooCommerceProduct {
    pub fn debug(&self) -> String {
        format!(
            "--- WOOCOMMERCE PRODUCT ---
NAME: {}
DESC: {}
CATEGORIES: {:?}
IMAGES URL: {:?}
STOCK_QTTY: {:?}
STATUS: {}
SERIAL: {}",
            self.name,
            self.description,
            self.categories,
            self.images,
            self.stock_quantity,
            self.status,
            self.sku
        )
    }
}

/*
pub name: String,
    pub regular_price: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub images: Vec<Image>,
    pub stock_quantity: Option<u32>,
    pub status: String,
    pub sku: String,
*/
