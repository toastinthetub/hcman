use dialoguer::Select;
use dotenv::dotenv;
use std::default;
use std::env;
use std::error::Error;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

use eframe::egui;

use crate::BasicEnv;
use crate::{
    local::{LocalObject, LocalSession},
    obj_vd::{ObjVendoo, VendooProduct},
    obj_wc::{ObjWooCommerce, WooCommerceProduct},
};

pub async fn init_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Centered Text Box with Buttons Example",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
    .unwrap_or_else(|e| eprintln!("Failed to start the GUI: {}", e));
}

#[derive(Default, Debug, Clone)]
pub enum SelectMode {
    #[default]
    WC,
    VD,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub env: BasicEnv,
    // pub api_base: String,
    // pub skey: String, // WC secret key, passed to ObjWc
    // pub ckey: String, // WC consumer key, passed to ObjWc

    // pub local_session: Option<LocalSession>,
    // pub csv_path: Option<String>, // csv path
    // pub local_db: Option<String>, // local db path
    // pub vd: Option<Arc<Mutex<ObjVendoo>>>,
    // pub wc: Option<Arc<Mutex<ObjWooCommerce>>>,

    // pub text_buffer: Arc<Mutex<String>>,
    // pub select_mode: Arc<Mutex<SelectMode>>,
    pub shared: Arc<Mutex<SharedData>>,
    pub wc_idx: i32,
    pub vd_idk: i32,

    pub initialized: bool,
}

#[derive(Default, Debug, Clone)]
pub struct SharedData {
    pub vd: Option<ObjVendoo>,
    pub wc: Option<ObjWooCommerce>,

    pub text_buffer: String,
    pub select_mode: SelectMode,
}

impl SharedData {
    pub async fn build(env: BasicEnv) -> Self {
        let mut vd = ObjVendoo::from_csv(&env.csv_path).unwrap();
        let mut wc = ObjWooCommerce::new_with_auth(env.wc_url, env.wc_ck, env.wc_sk);
        wc.fetch_populate_products().await.unwrap();

        let text_buffer = String::new();
        let select_mode = SelectMode::WC;

        Self {
            vd: Some(vd),
            wc: Some(wc),
            text_buffer,
            select_mode,
        }
    }
}

impl AppState {
    pub fn initialize(&mut self, env: BasicEnv) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new().unwrap();
        let shared_data = rt.block_on(SharedData::build(env));

        self.shared = Arc::new(Mutex::new(shared_data));

        let mut shared = self.shared.lock().unwrap();

        shared.text_buffer.clear();
        let _ = shared
            .text_buffer
            .write_str("WooCommerce DB over HTTP intialized, Vendoo DB deserialized from CSV.");
        // let wc = self.wc.lock().unwrap();
        // let vd = self.vd.lock().unwrap();

        // dotenv().ok();
        // let wc_api_url = env::var("WC_API_URL").expect("WC_API_URL not set");
        // let wc_consumer_key = env::var("WC_CONSUMER_KEY").expect("WC_CONSUMER_KEY not set");
        // let wc_consumer_secret =
        //     env::var("WC_CONSUMER_SECRET").expect("WC_CONSUMER_SECRET not set");
        // let mut csv_path: Option<String> = match env::var("CSV_PATH") {
        //     Ok(str) => Some(str),
        //     Err(_) => Some(crate::state::CSV_PATH_FAILED.to_owned()),
        // };
        // let mut local_db = env::var("LOCAL_DB").unwrap();

        // self.api_base = wc_api_url.clone();
        // self.skey = wc_consumer_secret.clone();
        // self.ckey = wc_consumer_key.clone();
        // self.local_db = Some(local_db);

        // rt.spawn(async move {
        //     match self.wc.unwrap().fetch_populate_products().await {
        //         Ok(lib) => lib,
        //         Err(e) => {
        //             eprintln!("failed to fetch products: {}", e);
        //         }
        //     };
        // });

        Ok(())
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Create a central panel to hold the main content
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.initialized {
                ui.label("INITIALIZING! FETCHING WC/VD INFORMATION & BUILDING SESSION");
                self.initialize(self.env.clone()).unwrap();
                self.initialized = true;
            }

            let mut shared = self.shared.lock().unwrap();

            // Top row: evenly spaced 4 buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = ui.available_width() / 6.0; // Spacing
                    for i in 1..=4 {
                        if ui.button(format!("Top Button {}", i)).clicked() {
                            println!("Top Button {} clicked", i);
                        }
                    }
                });
            });

            ui.add_space(20.0);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    // Left button
                    if ui.button("Left Button").clicked() {
                        println!("Left Button clicked");
                    }

                    // Centered read-only text
                    ui.add(
                        egui::TextEdit::multiline(&mut shared.text_buffer)
                            .desired_width(300.0)
                            .desired_rows(3)
                            .interactive(false),
                    );

                    // Right button
                    if ui.button("Right Button").clicked() {
                        println!("Right Button clicked");
                    }
                });
            });
        });
    }
}
