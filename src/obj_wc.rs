use base64::encode;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

    fn build_authorization_header(&self) -> String {
        let auth = format!("{}:{}", self.ckey, self.skey);
        let encoded_auth = encode(auth); // base64 encode
        format!("Basic {}", encoded_auth)
    }

    // Fetches all products and populates self.products
    pub async fn fetch_populate_products(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "{}/wp-json/wc/v3/products",
            self.base_api.trim_end_matches('/')
        );
        let client = Client::new();

        let auth_header = self.build_authorization_header();

        let response = client
            .get(&url)
            .header("Authorization", auth_header) // Manually add the Basic Auth header
            .send()
            .await?;

        if response.status().is_success() {
            // let products: Vec<WooCommerceProduct> = response.json().await?;
            // self.products = Some(products);
            // Ok(())
            let body = response.text().await?;

            let re = Regex::new(r"<[^>]*>").unwrap();
            let sanitized_body = re.replace_all(&body, "").to_string();

            let products: Vec<WooCommerceProduct> = serde_json::from_str(&sanitized_body)?;

            self.products = Some(products);
            Ok(())
        } else {
            let status = response.status();
            let error_msg = format!("Failed to fetch products: {}", status);
            Err(error_msg.into())
        }
    }

    // fetch and populate
    pub async fn fetch_products_raw(
        &self,
    ) -> Result<Vec<WooCommerceProduct>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/wp-json/wc/v3/products",
            self.base_api.trim_end_matches('/')
        );
        let client = Client::new();

        // build proper auth header
        let auth_header = self.build_authorization_header();

        let response = client
            .get(&url)
            .header("Authorization", auth_header) // put basicauth header in myself
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;

            // sanitize response body
            let re = Regex::new(r"<[^>]*>").unwrap();
            let sanitized_body = re.replace_all(&body, "").to_string();

            // deserialize json string sanitized
            let products: Vec<WooCommerceProduct> = serde_json::from_str(&sanitized_body)?;
            // let products: Vec<WooCommerceProduct> = response.json().await?;
            Ok(products)
        } else {
            let status = response.status();
            let error_msg = format!("Failed to fetch products: {}", status);
            Err(error_msg.into())
        }
    }
    pub async fn post_product(
        &self,
        product: WooCommerceProduct,
    ) -> Result<WooCommerceProduct, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/wp-json/wc/v3/products",
            self.base_api.trim_end_matches('/')
        );
        let client = Client::new();

        // gotta build auth header
        let auth_header = self.build_authorization_header();

        let response = client
            .post(&url)
            .header("Authorization", auth_header) // auth!
            .json(&product) // serialize
            .send()
            .await?;

        if response.status().is_success() {
            let created_product: WooCommerceProduct = response.json().await?;
            Ok(created_product)
        } else {
            let status = response.status();
            let error_msg = format!("Failed to create product: {}", status);
            Err(error_msg.into())
        }
    }
    pub fn get_length(&self) -> i32 {
        let length = self.products.as_ref().unwrap().len();
        length as i32
    }
}

impl WooCommerceProduct {
    pub fn debug(&self) -> String {
        // dbg single WC product
        let mut categories_str = String::new();
        let mut images_str = String::new();
        let mut stock_qty = String::new();

        if !self.categories.is_empty() {
            for category in self.categories.clone() {
                categories_str.push_str(&format!("{}, ", category.name));
            }
        } else {
            categories_str.push_str("N/A");
        }

        if !self.images.is_empty() {
            for image in self.images.clone() {
                images_str.push_str(&format!("{}, ", image.src));
            }
        } else {
            images_str.push_str("N/A");
        }

        if self.stock_quantity.is_none() {
            stock_qty.push_str("N/A");
        } else {
            stock_qty.push_str(&self.stock_quantity.unwrap().to_string());
        }

        format!(
            "--- WOOCOMMERCE PRODUCT ---
NAME: {}
DESC: {}
PRICE: {}
CATEGORIES: {}
IMAGES URL: {}
STOCK_QTTY: {}
STATUS: {}
SERIAL: {}",
            self.name,
            self.description,
            self.regular_price,
            categories_str,
            images_str,
            stock_qty,
            self.status,
            self.sku
        )
    }
}
