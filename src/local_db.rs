use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalProduct {
    pub name: String,
    pub sku: String,
    pub price: String,
    pub status: String,
}

pub fn load_local_db(file_path: &str) -> Result<Vec<LocalProduct>, Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        return Ok(Vec::new()); // if file doesnt exist just return empty
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let products: Vec<LocalProduct> = serde_json::from_reader(reader)?;

    Ok(products)
}

pub fn save_to_local_db(products: &[LocalProduct], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &products)?;

    Ok(())
}
