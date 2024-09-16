use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::models::VendooProduct;

pub fn read_vendoo_csv(file_path: &str) -> Result<Vec<VendooProduct>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    let mut products = Vec::new();
    
    for result in rdr.deserialize() {
        let product: VendooProduct = result?;
        products.push(product);
    }

    Ok(products)
}
