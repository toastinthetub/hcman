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
    pub async fn fetch_populate_products(
        // fetches all products from WC store and populates self.ProductVec
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/products", self.base_api);
        let client = Client::new();
        let response = client
            .get(&url)
            .basic_auth(self.ckey.clone(), Some(self.skey.clone()))
            .send()
            .await?;

        if response.status().is_success() {
            let products: Vec<WooCommerceProduct> = response.json().await?;
            self.products = Some(products);
            Ok(())
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }
    pub async fn fetch_products_raw(
        // just returns ProductVec
        &mut self,
    ) -> Result<Vec<WooCommerceProduct>, Box<dyn std::error::Error>> {
        let url = format!("{}/products", self.base_api);
        let client = Client::new();
        let response = client
            .get(&url)
            .basic_auth(self.ckey.clone(), Some(self.skey.clone()))
            .send()
            .await?;

        if response.status().is_success() {
            let products: Vec<WooCommerceProduct> = response.json().await?;
            Ok(products)
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
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
