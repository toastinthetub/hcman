use std::io::{self, Write};
use std::error::Error;
use crate::api::{fetch_all_products, post_products_from_csv};
use crate::vendoo::{VendooProduct, read_vendoo_csv, print_vendoo_product, print_all_vendoo_products};
use crate::local_db::{load_local_db, save_to_local_db, LocalProduct};

// this is in the wrong file but whatevs
fn compare_vendoo_to_db(vendoo_products: &[VendooProduct], local_products: &[LocalProduct]) {
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

pub async fn terminal_interface(api_url: &str, consumer_key: &str, consumer_secret: &str) -> Result<(), Box<dyn Error>> {
    let local_db_file = "woocommerce_db.json"; // file path, this will go in .env soon

    loop {
        println!("i know this is ugly, but select your option and press enter...");
        println!("1. fetch all products");
        println!("2. post products from CSV");
        println!("3. pick apart vendoo CSV");
        println!("4. compare Vendoo CSV to WooCommerce DB");
        println!("5. leave");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed to read, shitting pants and giving up");

        match choice.trim() {
            "1" => {
                // fetch prods and save to localdb
                let products = fetch_all_products(api_url, consumer_key, consumer_secret).await?;
                let local_products: Vec<LocalProduct> = products.iter().map(|p| {
                    LocalProduct {
                        name: p.name.clone(),
                        sku: p.sku.clone(),
                        price: p.regular_price.clone(),
                        status: p.status.clone(),
                    }
                }).collect();
                save_to_local_db(&local_products, local_db_file)?;
                println!("Fetched all products and saved them to the local database.");
            }
            "2" => {
                print!("enter CSV file path: ");
                io::stdout().flush().unwrap();
                let mut csv_file = String::new();
                io::stdin().read_line(&mut csv_file).expect("bad input");
                let csv_file = csv_file.trim();
                post_products_from_csv(csv_file, api_url, consumer_key, consumer_secret).await?;
            }
            "3" => {
                print!("enter CSV file path: ");
                io::stdout().flush().unwrap();
                let mut csv_file = String::new();
                io::stdin().read_line(&mut csv_file).expect("bad input");
                let csv_file = csv_file.trim();
                let products = read_vendoo_csv(&csv_file)?;
                print_all_vendoo_products(&products);
            }
            "4" => {
                // Compare Vendoo CSV products with WooCommerce DB
                print!("enter Vendoo CSV file path: ");
                io::stdout().flush().unwrap();
                let mut csv_file = String::new();
                io::stdin().read_line(&mut csv_file).expect("bad input");
                let csv_file = csv_file.trim();

                let vendoo_products = read_vendoo_csv(&csv_file)?;
                let local_products = load_local_db(local_db_file)?;

                compare_vendoo_to_db(&vendoo_products, &local_products);
            }
            "5" => {
                println!("bye!");
                break;
            }
            _ => {
                println!("bad input, try again");
            }
        }
    }

    Ok(())
}

// print products
pub fn print_products(products: Vec<crate::models::Product>) {
    for product in products {
        println!("ID: {}", product.name);
        println!("Price: {}", product.regular_price);
        println!("Stock: {:?}", product.stock_quantity);
        println!("Categories: {:?}", product.categories.iter().map(|c| &c.name).collect::<Vec<_>>());
        println!("Status: {}", product.status);
        println!("============================");
    }
}
