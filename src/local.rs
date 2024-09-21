use crate::{
    obj_vd::{ExternalImage, ObjVendoo, VendooProduct},
    obj_wc::ObjWooCommerce,
};

pub struct LocalObject {
    // VendooProduct and WooCommerceProduct will both turn into this.
    pub name: String,
    pub regular_price: String,
    pub description: String,
    pub categories: Vec<String>,
    pub images: Vec<String>,
    pub stock_quantity: Option<u32>,
    pub status: String,
    pub sku: String,
}

impl LocalObject {
    pub fn from_vendoo_object(vprod: &VendooProduct) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }
}
