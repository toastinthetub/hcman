use dotenv::dotenv;
use std::default;
use std::env;
use std::error::Error;
use tokio::runtime::Runtime;

use eframe::egui;

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

#[derive(Default, Debug)]
pub enum SelectMode {
    #[default]
    WC,
    VD,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub api_base: String,
    pub skey: String, // WC secret key, passed to ObjWc
    pub ckey: String, // WC consumer key, passed to ObjWc

    pub local_session: Option<LocalSession>,
    pub csv_path: Option<String>, // csv path
    pub local_db: Option<String>, // local db path
    pub vd: Option<ObjVendoo>,
    pub wc: Option<ObjWooCommerce>,

    pub text_buffer: String,
    pub select_mode: SelectMode,
    pub wc_idx: i32,
    pub vd_idk: i32,

    pub initialized: bool,
}

impl AppState {
    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new().unwrap();

        dotenv().ok();
        let wc_api_url = env::var("WC_API_URL").expect("WC_API_URL not set");
        let wc_consumer_key = env::var("WC_CONSUMER_KEY").expect("WC_CONSUMER_KEY not set");
        let wc_consumer_secret =
            env::var("WC_CONSUMER_SECRET").expect("WC_CONSUMER_SECRET not set");
        let mut csv_path: Option<String> = match env::var("CSV_PATH") {
            Ok(str) => Some(str),
            Err(_) => Some(crate::state::CSV_PATH_FAILED.to_owned()),
        };
        let mut local_db = env::var("LOCAL_DB").unwrap();

        self.api_base = wc_api_url.clone();
        self.skey = wc_consumer_secret.clone();
        self.ckey = wc_consumer_key.clone();
        self.local_db = Some(local_db);

        // rt.spawn(async {
        //     match self.wc.as_mut().unwrap().fetch_populate_products().await {
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
            }

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

            // Add vertical space to separate the top buttons and the center section
            ui.add_space(20.0);

            // Center row: text box with left and right buttons
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    // Left button
                    if ui.button("Left Button").clicked() {
                        println!("Left Button clicked");
                    }

                    // Centered read-only text
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text_buffer)
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
