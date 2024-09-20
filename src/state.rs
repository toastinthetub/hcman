use crate::{obj_vd::ObjVendoo, obj_wc::ObjWooCommerce};
use dialoguer::{Input, Select};

pub struct State {
    pub api_base: String,
    pub skey: String, // WC secret key, passed to ObjWc
    pub ckey: String, // WC consumer key, passed to ObjWc
    pub test: bool,

    pub csv_path: Option<String>, // csv path
    pub local_db: Option<String>, // local db path
    pub vd: Option<ObjVendoo>,
    pub wc: Option<ObjWooCommerce>,
}

impl State {
    pub async fn prod_pipeline(&mut self) {}
}

impl State {
    // testing impl
    pub async fn test_pipeline(&mut self) {
        println!("[] POPULATING ObjWc & ObjVd...");

        self.wc = Some(ObjWooCommerce::new_with_auth(
            self.api_base.clone(),
            self.ckey.clone(),
            self.skey.clone(),
        ));

        let _ = self.wc.clone().unwrap().fetch_populate_products().await;
        println!("wc populated");

        // --- TODO --- vendoo! read from CSV

        loop {
            let option = Select::new()
                .with_prompt("HCRELAY TEST MENU")
                .items(&[
                    "WooCommerce Options",
                    "Database Options",
                    "Vendoo Options",
                    "Exit",
                ])
                .default(0)
                .interact()
                .unwrap();

            match option {
                0 => {
                    // WooCommerce options
                    let _ = self.wc_options_term().await.unwrap();
                }
                1 => {
                    println!("\n --- todo! ---")
                }
                2 => {
                    println!("\n--- todo! ---")
                }
                3 => {
                    println!("\nbye!");
                    std::process::exit(0);
                    break;
                }
                _ => {
                    //
                }
            }
        }
    }

    pub async fn wc_options_term(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let option = Select::new()
                .with_prompt("WOOCOMMERCE MENU")
                .items(&[
                    "Fetch WooCommerce Library",
                    "Post a VendooProduct to WooCommerce",
                    "Batch upload Vendoo CSV to WooCommerce",
                    "Back",
                    "Exit",
                ])
                .default(0)
                .interact()
                .unwrap();

            match option {
                0 => {
                    println!("--- fetching WooCommerce lib ---");
                    let lib = match self.wc.clone().unwrap().fetch_products_raw().await {
                        Ok(lib) => lib,
                        Err(e) => {
                            println!("this fucking error happened! {e}");
                            return Err(e);
                        }
                    };
                    let option = Select::new()
                        .with_prompt("The WooCommerce lib has been fetched. See now?")
                        .items(&["Yes", "No", "Back", "Exit"])
                        .default(0)
                        .interact()
                        .unwrap();

                    println!("{:?}", lib);

                    // --- TODO! ---
                }
                1 => {
                    // TODO!
                    todo!()
                }
                2 => {
                    // TODO!
                    todo!()
                }
                3 => {
                    // go back to last menu!
                    break;
                }
                4 => {
                    println!("bye!");
                    std::process::exit(0);
                }
                _ => {
                    //
                }
            }
        }
        return Ok(());
    }
}
