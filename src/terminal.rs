use std::error::Error;
use crate::api::{fetch_all_products, post_products_from_csv};
use crate::vendoo::{VendooProduct, read_vendoo_csv, print_vendoo_product, print_all_vendoo_products};
use crate::local_db::{load_local_db, save_to_local_db, LocalProduct};

// refactor existing logic into utility functions for GUI integration
pub async fn fetch_products_and_save(api_url: &str, consumer_key: &str, consumer_secret: &str) -> Result<(), Box<dyn Error>> {
    let products = fetch_all_products(api_url, consumer_key, consumer_secret).await?;
    let local_products: Vec<LocalProduct> = products.iter().map(|p| {
        LocalProduct {
            name: p.name.clone(),
            sku: p.sku.clone(),
            price: p.regular_price.clone(),
            status: p.status.clone(),
        }
    }).collect();
    save_to_local_db(&local_products, "woocommerce_db.json")?;
    Ok(())
}

pub fn compare_vendoo_to_db(vendoo_products: &[VendooProduct], local_products: &[LocalProduct]) {
    for vendoo_product in vendoo_products {
        let vendoo_sku = vendoo_product.sku.as_deref().unwrap_or("N/A");
        let vendoo_title = vendoo_product.title.as_deref().unwrap_or("N/A");

        if local_products.iter().any(|p| p.sku == vendoo_sku) {
            println!("Vendoo Product '{}' (SKU: {}) is already in WooCommerce", vendoo_title, vendoo_sku);
        } else {
            println!("Vendoo Product '{}' (SKU: {}) is NOT in WooCommerce", vendoo_title, vendoo_sku);
        }
    }
}
