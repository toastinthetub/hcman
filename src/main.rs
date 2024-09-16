mod api;
mod models;
mod vendoo;

use dotenv::dotenv;
use eframe::egui;
use std::env;
use std::error::Error;
use crate::models::{Product, VendooProduct};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let wc_api_url = env::var("WC_API_URL").expect("WC_API_URL not set");
    let wc_consumer_key = env::var("WC_CONSUMER_KEY").expect("WC_CONSUMER_KEY not set");
    let wc_consumer_secret = env::var("WC_CONSUMER_SECRET").expect("WC_CONSUMER_SECRET not set");

    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "WooCommerce Manager",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(wc_api_url, wc_consumer_key, wc_consumer_secret)))),
    );

    Ok(())
}

#[derive(Clone)]  
struct MyApp {
    api_url: String,
    consumer_key: String,
    consumer_secret: String,
    products: Vec<Product>,           // WooCommerce products
    current_product_index: usize,      // Index to track the current product
    vendoo_products: Vec<VendooProduct>, // Vendoo CSV products
    vendoo_index: usize,               // Index to track current Vendoo product
    result_text: String,               // Display product info here
}

impl MyApp {
    fn new(api_url: String, consumer_key: String, consumer_secret: String) -> Self {
        Self {
            api_url,
            consumer_key,
            consumer_secret,
            products: vec![],
            current_product_index: 0,
            vendoo_products: vec![],
            vendoo_index: 0,
            result_text: String::new(),
        }
    }

    fn display_wc_product(&self) -> String {
        if let Some(product) = self.products.get(self.current_product_index) {
            format!(
                "Product Name: {}\nPrice: {}\nStock: {:?}\nStatus: {}\nSKU: {}\nDescription: {}\n",
                product.name, product.regular_price, product.stock_quantity, product.status, product.sku, product.description
            )
        } else {
            "No WooCommerce products loaded.".to_string()
        }
    }

    fn display_vendoo_product(&self) -> String {
        if let Some(product) = self.vendoo_products.get(self.vendoo_index) {
            format!(
                "Vendoo Product: {}\nPrice: {}\nSKU: {}\nCondition: {}\nCategory: {}\n",
                product.title.as_deref().unwrap_or("N/A"),
                product.price.as_deref().unwrap_or("N/A"),
                product.sku.as_deref().unwrap_or("N/A"),
                product.condition.as_deref().unwrap_or("N/A"),
                product.category.as_deref().unwrap_or("N/A")
            )
        } else {
            "No Vendoo CSV products loaded.".to_string()
        }
    }

    async fn load_wc_products(&mut self) {
        match api::fetch_all_products(&self.api_url, &self.consumer_key, &self.consumer_secret).await {
            Ok(products) => {
                self.products = products;
                self.result_text = self.display_wc_product();
            }
            Err(e) => {
                self.result_text = format!("Error loading WooCommerce products: {:?}", e);
            }
        }
    }

    fn load_vendoo_csv(&mut self, csv_file: &str) {
        match vendoo::read_vendoo_csv(csv_file) {
            Ok(products) => {
                self.vendoo_products = products;
                self.result_text = self.display_vendoo_product();
            }
            Err(e) => {
                self.result_text = format!("Error loading Vendoo CSV: {:?}", e);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WooCommerce and Vendoo Manager");

            if ui.button("Fetch WooCommerce Products").clicked() {
                let api_url = self.api_url.clone();
                let consumer_key = self.consumer_key.clone();
                let consumer_secret = self.consumer_secret.clone();
                let mut app_clone = self.clone();
                            tokio::spawn(async move {
                    app_clone.load_wc_products().await;
                });
            }

            if !self.products.is_empty() {
                if ui.button("Previous WooCommerce Product").clicked() {
                    if self.current_product_index > 0 {
                        self.current_product_index -= 1;
                    }
                    self.result_text = self.display_wc_product();
                }
                if ui.button("Next WooCommerce Product").clicked() {
                    if self.current_product_index < self.products.len() - 1 {
                        self.current_product_index += 1;
                    }
                    self.result_text = self.display_wc_product();
                }
            }

            ui.separator();
            ui.label("Enter Vendoo CSV file path: ");
            let mut csv_file = String::new();
            if ui.text_edit_singleline(&mut csv_file).changed() && !csv_file.is_empty() {
                self.load_vendoo_csv(&csv_file);
            }

            if !self.vendoo_products.is_empty() {
                if ui.button("Previous Vendoo Product").clicked() {
                    if self.vendoo_index > 0 {
                        self.vendoo_index -= 1;
                    }
                    self.result_text = self.display_vendoo_product();
                }
                if ui.button("Next Vendoo Product").clicked() {
                    if self.vendoo_index < self.vendoo_products.len() - 1 {
                        self.vendoo_index += 1;
                    }
                    self.result_text = self.display_vendoo_product();
                }
            }

            
            ui.add(egui::TextEdit::multiline(&mut self.result_text).desired_width(f32::INFINITY).desired_rows(10));

            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }
}
