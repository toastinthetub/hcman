use std::collections::HashSet;
use std::io::Read;

use hex::encode;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{
    obj_vd::{ExternalImage, ObjVendoo, VendooProduct},
    obj_wc::{self, Category, Image, ObjWooCommerce, WooCommerceProduct},
};

#[derive(Debug, Deserialize, Clone)]
pub enum Sig {
    WC,
    VD,
}

#[derive(Debug, Deserialize, Clone)]
pub enum JsonBuffer {
    LocalBuffer,
    RemoteBuffer,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalSession {
    pub n: i32,
    pub local_vp: Vec<LocalObject>, // will have differing signatures
    pub local_wp: Vec<LocalObject>, // will have differing signatures
    pub current_idx: i32,
}

impl LocalSession {
    pub fn from_local_products(vlp: Vec<LocalObject>) -> Self {
        let mut n: i32 = 0;
        let mut local_vp: Vec<LocalObject> = Vec::new();
        let mut local_wp: Vec<LocalObject> = Vec::new();
        let current_idx: i32 = 0;

        for lp in vlp {
            n += 1;

            match lp.sig {
                Sig::WC => {
                    // wc product push 2 local_wp
                    local_wp.push(lp);
                }
                Sig::VD => {
                    // vd product push 2 local_vd
                    local_vp.push(lp);
                }
            }
        }
        Self {
            n,
            local_vp,
            local_wp,
            current_idx,
        }
    }

    pub fn from_local_json(filepath: &str) -> Self {
        let mut n: i32 = 0;
        let mut local_vp: Vec<LocalObject> = Vec::new();
        let mut local_wp: Vec<LocalObject> = Vec::new();
        let current_idx: i32 = 0;

        let mut f = std::fs::File::open(filepath).unwrap(); // TODO fix this
        let mut json = String::new();
        f.read_to_string(&mut json).unwrap();

        let local_objects: Vec<LocalObject> = serde_json::from_str(&json).unwrap_or(Vec::new());

        for local_object in local_objects {
            n += 1;

            match local_object.sig {
                Sig::WC => {
                    // wc product push 2 local_wp
                    local_wp.push(local_object);
                }
                Sig::VD => {
                    // vd product push 2 local_vd
                    local_vp.push(local_object);
                }
            }
        }

        Self {
            n,
            local_vp,
            local_wp,
            current_idx,
        }
    }

    pub fn from_session(wp_obj: ObjWooCommerce, vd_obj: ObjVendoo) -> Self {
        let mut n: i32 = 0;
        let mut local_vp: Vec<LocalObject> = Vec::new();
        let mut local_wp: Vec<LocalObject> = Vec::new();
        let current_idx: i32 = 0;

        for object in wp_obj.products.unwrap_or(Vec::new()) {
            n += 1;
            let lwp = LocalObject::from_woocommerce_object(&object);
            local_wp.push(lwp);
        }

        for object in vd_obj.products.unwrap_or(Vec::new()) {
            n += 1;
            let vdp = LocalObject::from_vendoo_object(&object);
            local_vp.push(vdp);
        }

        Self {
            n,
            local_vp,
            local_wp,
            current_idx,
        }
    }
    pub fn compare_wc_vd(&self) -> (i32, Vec<LocalObject>) {
        let mut matched: i32 = 0;
        let mut need_posted: Vec<LocalObject> = Vec::new();

        let wp_hashes: HashSet<_> = self
            .local_wp
            .iter()
            .map(|wp_object| &wp_object.hash_hex)
            .collect();

        for vp_object in self.local_vp.clone() {
            if vp_object.status == "Active" {
                if wp_hashes.contains(&vp_object.hash_hex) {
                    matched += 1;
                    println!(
                        "wc: {} matches vd: {}!",
                        vp_object.hash_hex, vp_object.hash_hex
                    );
                } else {
                    need_posted.push(vp_object);
                }
            }
        }

        (matched, need_posted)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalObject {
    // VendooProduct and WooCommerceProduct will both turn into this.
    pub sig: Sig,
    pub hash_hex: String,
    pub name: String,
    pub regular_price: String,
    pub description: String,
    pub categories: String,
    pub images: Vec<String>,
    pub stock_quantity: Option<u32>,
    pub status: String,
    pub sku: String,
}

impl LocalObject {
    //converts vendoo object to local object
    pub fn from_vendoo_object(vprod: &VendooProduct) -> Self {
        let sig: Sig = Sig::VD;
        let images = vprod.images.clone().unwrap_or(String::new());
        let images: Vec<String> = vec![images];
        let name = vprod.title.clone().unwrap_or(String::new());
        let description = vprod.description.clone().unwrap_or(String::new());
        let sku = vprod.sku.clone().unwrap_or(String::new());
        let category = vprod.category.clone().unwrap_or(String::new());
        let price = vprod.price.clone().unwrap_or(0.0);
        let status = vprod.status.clone().unwrap_or(String::new());
        let mut stock_qty: Option<u32> = Some(vprod.quantity_left.clone().unwrap_or(0));
        if stock_qty == Some(0) {
            stock_qty = None
        }

        let mut hash_hex = String::new();

        if name.is_empty() {
            hash_hex.push_str("NO TITLE, NO HASH ID");
        } else {
            let mut hasher = Sha256::new();
            hasher.update(name.clone());
            let res = hasher.finalize();
            hash_hex = hex::encode(res);
        }

        Self {
            sig,
            hash_hex,
            name: name,
            regular_price: price.to_string(),
            description,
            categories: category,
            images: images,
            stock_quantity: stock_qty,
            status,
            sku,
        }
    }

    pub fn from_woocommerce_object(wprod: &WooCommerceProduct) -> Self {
        // converts wc object to localobject
        let sig: Sig = Sig::WC;
        let mut images: Vec<String> = Vec::new();
        for image in wprod.images.clone() {
            images.push(image.src)
        }
        let name = wprod.name.clone();
        let regular_price = wprod.regular_price.clone();
        let description = wprod.description.clone();
        let mut categories: String = String::new();
        for category in wprod.categories.clone() {
            categories.push_str(&format!("{}, ", category.name));
        }
        let mut stock_qty: Option<u32> = Some(wprod.stock_quantity.unwrap_or(0));
        if stock_qty == Some(0) {
            stock_qty = None
        }
        let status = wprod.status.clone();
        let sku = wprod.sku.clone();

        let mut hash_hex: String = String::new();

        if name.is_empty() {
            hash_hex.push_str("NO TITLE, NO HASH ID");
        } else {
            let mut hasher = Sha256::new();
            hasher.update(name.clone());
            let res = hasher.finalize();
            hash_hex = hex::encode(res);
        }

        Self {
            sig,
            hash_hex,
            name: name,
            regular_price,
            description,
            categories,
            images: images,
            stock_quantity: stock_qty,
            status,
            sku,
        }
    }

    /*
    pub sig: Sig,
    pub hash_hex: String,
    pub name: String,
    pub regular_price: String,
    pub description: String,
    pub categories: String,
    pub images: Vec<String>,
    pub stock_quantity: Option<u32>,
    pub status: String,
    pub sku: String,


    pub name: String,
    pub regular_price: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub images: Vec<Image>,
    pub stock_quantity: Option<u32>,
    pub status: String,
    pub sku: String,
    */

    pub fn to_woocommerce_object(&mut self) -> WooCommerceProduct {
        let mut images: Vec<Image> = Vec::new();
        for str in self.images.clone() {
            let image = Image { src: str };
            images.push(image);
        }
        let mut categories: Vec<Category> = Vec::new();
        let split: Vec<String> = self.categories.split(',').map(|s| s.to_string()).collect();
        for str in split {
            let category = Category { name: str };
            categories.push(category)
        }

        WooCommerceProduct {
            name: self.name.clone(),
            regular_price: self.regular_price.clone(),
            description: self.description.clone(),
            categories,
            images,
            stock_quantity: self.stock_quantity,
            status: self.status.clone(),
            sku: self.sku.clone(),
        }
    }
}
