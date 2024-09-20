use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ObjVendoo {
    pub csv_path: Option<String>,                 // path to CSV
    pub products: Option<Vec<VendooProduct>>,     // big ol' impl ObjVendoo
    pub external_img: Option<Vec<ExternalImage>>, // urls for images, each with product ID
}

#[derive(Debug, Deserialize, Clone)]
pub struct VendooProduct {
    #[serde(rename = "Images")]
    pub images: Option<String>,

    #[serde(rename = "Title")]
    pub title: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "Brand")]
    pub brand: Option<String>,

    #[serde(rename = "Condition")]
    pub condition: Option<String>,

    #[serde(rename = "Primary Color")]
    pub primary_color: Option<String>,

    #[serde(rename = "Secondary Color")]
    pub secondary_color: Option<String>,

    #[serde(rename = "Tags")]
    pub tags: Option<String>,

    #[serde(rename = "Sku")]
    pub sku: Option<String>,

    #[serde(rename = "Category")]
    pub category: Option<String>,

    #[serde(rename = "Price")]
    pub price: Option<f64>,

    #[serde(rename = "Status")]
    pub status: Option<String>,

    #[serde(rename = "Listed Date")]
    pub listed_date: Option<String>,

    #[serde(rename = "Sold Date")]
    pub sold_date: Option<String>,

    #[serde(rename = "Shipped Date")]
    pub shipped_date: Option<String>,

    #[serde(rename = "Listing Platforms")]
    pub listing_platforms: Option<String>,

    #[serde(rename = "Sold Platform")]
    pub sold_platform: Option<String>,

    #[serde(rename = "Internal Notes")]
    pub internal_notes: Option<String>,

    #[serde(rename = "Price Sold")]
    pub price_sold: Option<f64>,

    #[serde(rename = "Cost of Goods")]
    pub cost_of_goods: Option<f64>,

    #[serde(rename = "Marketplace Fees")]
    pub marketplace_fees: Option<f64>,

    #[serde(rename = "Shipping Expenses")]
    pub shipping_expenses: Option<f64>,

    #[serde(rename = "Labels")]
    pub labels: Option<String>,

    #[serde(rename = "Quantity Left")]
    pub quantity_left: Option<u32>,

    #[serde(rename = "Quantity Sold")]
    pub quantity_sold: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExternalImage {
    pub src: String,
}

impl ObjVendoo {
    pub fn empty() -> Self {
        // generate empty self
        Self {
            csv_path: None,
            products: Some(Vec::new()),
            external_img: None,
        }
    }
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // generates an ObjVendoo without any external images.
        let file = File::open(Path::new(path))?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        let mut vendoo_products: Vec<VendooProduct> = Vec::new();
        let mut x = 0;
        let mut y = 0;

        for result in rdr.deserialize() {
            match result {
                Ok(product) => {
                    vendoo_products.push(product);
                    x += 1;
                    y += 1;
                }
                Err(e) => {
                    eprintln!("[] Error parsing record: {:?}", e); // print any error (sucks!)
                    x += 1;
                }
            }
        }

        println!("[] successfully parsed {} of {} rows", y, x);

        Ok(Self {
            csv_path: Some(path.to_owned()),
            products: Some(vendoo_products),
            external_img: None,
        })
    }

    pub fn existing_from_csv(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(Path::new(path))?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        let mut vendoo_products: Vec<VendooProduct> = Vec::new();
        let mut x = 0;
        let mut y = 0;

        for result in rdr.deserialize() {
            match result {
                Ok(product) => {
                    vendoo_products.push(product);
                    x += 1;
                    y += 1;
                }
                Err(e) => {
                    eprintln!("[] Error parsing record: {:?}", e); // print any error (sucks!)
                    x += 1;
                }
            }
        }

        self.csv_path = Some(path.to_owned());
        self.products = Some(vendoo_products);

        println!("[] successfully parsed {} of {} rows", y, x);

        Ok(())
    }
}
