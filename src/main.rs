mod local;
mod obj_vd;
mod obj_wc;
mod state;
mod utils;

use dotenv::dotenv;
use obj_vd::ObjVendoo;
use serde::{Deserialize, Serialize};
use std::{
    env::{self, args},
    sync::{Arc, Mutex},
};
use utils::init_gui;

use crate::state::State;

#[derive(Default, Debug, Clone)]
pub struct BasicEnv {
    wc_url: String,
    wc_ck: String,
    wc_sk: String,
    csv_path: String,
    json_path: String,
}

// #[tokio::main]
/*async*/
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file: String = String::new(); // csv file

    let idx: usize = 2; // magic number ik chill out

    let mut test: bool = match args.get(1) {
        Some(mut str) => {
            match str.as_str() {
                "-h" | "--help" => {
                    println!(
                        "usage: hcrelay [-t | --testing] [-h | --help] [-f <path> | --file <path>]"
                    );
                    std::process::exit(0);
                    // false
                }
                "-t" | "--testing" => {
                    println!("HCRELAY SAYS testing mode enabled");
                    true
                }
                "-f" | "--file" => {
                    str = args.get(idx as usize + 1).unwrap();
                    file = str.to_owned();
                    false
                }
                _ => {
                    println!("argument {} not understood", str);
                    println!("usage: hcrelay [-t | --testing] [-h | --help]");
                    std::process::exit(0);
                    // false
                }
            }
        }
        None => false,
    };

    file = match args.get(idx) {
        Some(mut str) => {
            match str.as_str() {
                "-h" | "--help" => {
                    println!(
                        "usage: hcrelay [-t | --testing] [-h | --help] [-f <path> | --file <path>]"
                    );
                    std::process::exit(0);
                    // false
                }
                "-t" | "--testing" => {
                    println!("HCRELAY SAYS testing mode enabled");
                    test = true;
                    "".to_owned()
                }
                "-f" | "--file" => {
                    str = args.get(idx as usize + 1).unwrap();
                    str.to_owned()
                }
                _ => {
                    println!("argument {} not understood", str);
                    println!(
                        "usage: hcrelay [-t | --testing] [-h | --help] [-f <path> | --file <path>]"
                    );
                    std::process::exit(0);
                    // false
                }
            }
        }
        None => "".to_owned(),
    };

    dotenv().ok();
    let wc_api_url = env::var("WC_API_URL").expect("WC_API_URL not set");
    let wc_consumer_key = env::var("WC_CONSUMER_KEY").expect("WC_CONSUMER_KEY not set");
    let wc_consumer_secret = env::var("WC_CONSUMER_SECRET").expect("WC_CONSUMER_SECRET not set");
    let mut csv_path: String = match env::var("CSV_PATH") {
        Ok(str) => {
            file = str.clone();
            str
        }
        Err(_) => file.clone(),
    };
    let local_db = env::var("LOCAL_DB").unwrap_or(String::from("no local db!"));

    if csv_path != "".to_string() {
        csv_path = crate::state::CSV_PATH_FAILED.to_string();
    }

    let mut local_db: Option<String> = match env::var("LOCAL_DB") {
        Ok(str) => Some(str),
        Err(_) => Some(file.clone()),
    };

    let basic_auth = BasicEnv {
        wc_url: wc_api_url.clone(),
        wc_ck: wc_consumer_key.clone(),
        wc_sk: wc_consumer_secret.clone(),

        csv_path: csv_path.clone(),
        json_path: local_db.clone().unwrap(),
    };

    // let logger_fn = |message: &str| println!("{:?}", message);

    let mut state: State = State {
        api_base: wc_api_url,
        skey: wc_consumer_secret,
        ckey: wc_consumer_key,
        test: test,

        csv_path: Some(csv_path),
        local_db: local_db,

        vd: None,
        wc: None,
    };

    match state.test {
        true => {
            println!("[] starting in test mode!");
            // state.test_pipeline().await;
        }
        false => {
            println!("[] todo! sorry lol");
            println!("[] launching anyway in test mode");
            println!("[] just kidding we're doing sams OTHER test mode!");
            println!("[] double just kidding gui!");
            init_gui();
            // let _ = state.sams_crazy_test_pipeline().await;
        }
    }
}

/*
what do i want...

i want to curl a certain url to get a CSV file from vendoo. auth is going to be a
pain in my ass i think. manual for now until better idea .I need to figure out exactly
what each header is and they all need to be Option<String> because a bunch of them don't
have it. i need an option to test. maybe -t / --testing for TUI mode. deserialize each
value into CSV object. slice and dice for WooCommerce export and present Vec<ProductWC>.
each ProductWC instance can be posted to WooCommerce. ProductWC constructor needs to
generate endpoint for products of any size. also need to find space for image db over http.

yes i've got it pipeline

pub struct State {      // fill later
    api_base: String
    skey: String        // api_base and keys from env
    ckey: String
    test: bool          // will bypass sleep
    logger: Fn(&str)
    -- todo --
}

impl State {
    -- todo --
}

pub struct ObjVendoo {
    csv_path: Option<String> // path to CSV
    products: Option<Vec<VendooProduct>> // big ol' impl ObjVendoo
    external_img: Option<Vec<ExternalImage>> // urls for images, each with product ID
}

impl ObjV {
    pub fn new_empty
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn std::error::Error>> {} // essentially
    -- todo --
}

struct VendooProduct {
    // VENDOO BS
}

impl VendooProduct {
    pub fn
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error> {} // ess
}
*/
