use std::io::Write;

use crate::{
    local::{self, LocalObject, LocalSession},
    obj_vd::ObjVendoo,
    obj_wc::ObjWooCommerce,
};
use dialoguer::{Input, Select};

const CSV_PATH_FAILED: &str = "/home/fizbin/lair/proj/rust/hcrelay/asset/vendoo.csv";

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
    pub async fn prod_pipeline(&mut self) {
        //
    }
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

        self.vd = Some(
            ObjVendoo::from_csv(&self.csv_path.clone().unwrap_or(CSV_PATH_FAILED.to_owned()))
                .unwrap(),
        );

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
                    "TESTPIPELINE!",
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
                    println!("todo!")
                }
                2 => {
                    let _ = self.vd_options_term().await.unwrap();
                }
                3 => {
                    // todo!
                }
                4 => {
                    println!("\nbye!");
                    std::process::exit(0);
                    // break;
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
                            return Err(e);
                        }
                    };
                    let option = Select::new()
                        .with_prompt("The WooCommerce lib has been fetched. See now?")
                        .items(&["Yes", "No", "Back", "Exit"])
                        .default(0)
                        .interact()
                        .unwrap();

                    // println!("{:?}", lib);

                    if option == 0 {
                        for object in lib
                        /*self.wc.clone().unwrap().products.unwrap()*/
                        {
                            println!("{}", object.debug());
                        }
                        let mut s = String::new();
                        std::io::stdin().read_line(&mut s).unwrap();
                        std::mem::drop(s);
                    }

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

    pub async fn vd_options_term(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let option = Select::new()
                .with_prompt("VENDOO MENU")
                .items(&[
                    "Display all Vendoo products",
                    "General CSV information",
                    "Back",
                    "Exit",
                ])
                .default(0)
                .interact()
                .unwrap();

            match option {
                0 => {
                    println!("--- deserializing Vendoo products... ---");

                    if self.vd.is_none() {
                        if !self.csv_path.is_some() {
                            self.csv_path = Some(
                                Input::<String>::new()
                                    .with_prompt(
                                        "No CSV file has been configured. Enter CSV file path:",
                                    )
                                    .interact_text()
                                    .unwrap(),
                            );
                        } else {
                            self.vd =
                                Some(ObjVendoo::from_csv(&self.csv_path.clone().unwrap()).unwrap());
                        }
                    } else {
                        if self.vd.clone().unwrap().products.is_none() {
                            self.vd
                                .as_mut()
                                .expect("no vd!")
                                .existing_from_csv(&self.csv_path.clone().unwrap())
                                .unwrap()
                            // reconstruct
                        }
                    }

                    let option = Select::new()
                        .with_prompt("The Vendoo lib has been fetched. See now?")
                        .items(&["Yes", "No", "Back", "Exit"])
                        .default(0)
                        .interact()
                        .unwrap();

                    if option == 0 {
                        for vendoo_prod in self.vd.clone().unwrap().products.unwrap() {
                            println!("{:?}", vendoo_prod);
                        }
                    }

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

impl State {
    pub async fn sams_crazy_test_pipeline(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        press_enter_to_continue(String::from("create new wc and vendoo!"));

        println!("[]creating new wc");
        self.wc = Some(ObjWooCommerce::new_with_auth(
            self.api_base.clone(),
            self.ckey.clone(),
            self.skey.clone(),
        ));
        println!("done");

        println!("[]populating instance from store");
        match self.wc.as_mut().unwrap().fetch_populate_products().await {
            Ok(lib) => lib,
            Err(e) => {
                return Err(e);
            }
        };
        println!("done");

        println!("[]populating lib var from store");
        let lib = match self.wc.clone().unwrap().fetch_products_raw().await {
            Ok(lib) => lib,
            Err(e) => {
                return Err(e);
            }
        };

        println!("done");

        println!("[]creating new vd from csv");
        self.vd = Some(
            ObjVendoo::from_csv(&self.csv_path.clone().unwrap_or(CSV_PATH_FAILED.to_owned()))
                .unwrap(),
        );
        println!("done");

        press_enter_to_continue(String::from("debug raw woocommerce objects"));

        let mut idx: i32 = 0;

        for obj in lib {
            println!("WC RAW OBJECT #{}:\n{}", idx, obj.debug());
            idx += 1;
        }

        idx = 0;

        press_enter_to_continue(String::from("debug *parsed* woocommerce objects"));

        for obj in self.wc.clone().unwrap().products.unwrap() {
            println!("WC PARSED OBJECT #{}\n{}", idx, obj.debug());
            idx += 1;
        }

        idx = 0;

        press_enter_to_continue(String::from("debug woocommerce entries"));

        for obj in self.vd.clone().unwrap().products.unwrap() {
            println!("VENDOO PARSED OBJECT #{}\n{}", idx, obj.debug());
            idx += 1;
        }

        idx = 0;

        press_enter_to_continue(String::from("create LocalSession"));

        let mut local_session =
            LocalSession::from_session(self.wc.clone().unwrap(), self.vd.clone().unwrap());

        press_enter_to_continue(String::from("print out all wc_vec Titles, IDX and SKU's"));

        let wp_len = local_session.local_wp.len();
        let mut x = 0;
        let vp_len = local_session.local_vp.len();
        let mut y = 0;

        for obj in local_session.local_wp {
            println!("WC Object: #{}\nTitle: {}\nSKU: {}", x, obj.name, obj.sku);
            x += 1;
        }
        println!("printed {} objects of {} in array", x, wp_len);

        press_enter_to_continue(String::from("print out all vp_vec Titles, IDX and SKU'S"));

        for obj in local_session.local_vp {
            println!("VD Object: #{}\nTitle: {}\nSKU: {}", y, obj.name, obj.sku);
            y += 1;
        }

        Ok(())
    }
}

pub fn press_enter_to_continue(str: String) {
    let mut s = String::new();
    let mut stdout = std::io::stdout();
    print!("press enter when ready to: {}", str);
    stdout.flush().unwrap();
    std::io::stdin().read_line(&mut s).unwrap();
    std::mem::drop(s);
}
