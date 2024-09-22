use dialoguer::Select;
use dotenv::dotenv;
use eframe::egui::Align;
use eframe::egui::Button;
use eframe::egui::CentralPanel;
use eframe::egui::Label;
use eframe::egui::Layout;
use eframe::egui::Pos2;
use eframe::egui::Rect;
use eframe::egui::TextEdit;
use eframe::egui::Vec2;
use sha2::digest::consts::True;
use std::default;
use std::env;
use std::error::Error;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

use eframe::egui;

use crate::state;
use crate::BasicEnv;
use crate::{
    local::{LocalObject, LocalSession},
    obj_vd::{ObjVendoo, VendooProduct},
    obj_wc::{ObjWooCommerce, WooCommerceProduct},
};

pub fn init_gui() {
    println!("init gui was in fact called.");
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vendoo -> WooCommerce Crosslister.",
        options,
        Box::new(|_cc| Ok(Box::<AppState>::default())),
    )
    .unwrap_or_else(|e| eprintln!("Failed to start the GUI: {}", e));
}

#[derive(Default, Debug, Clone)]
pub enum SelectMode {
    #[default]
    WC,
    VD,
}

#[derive(Debug)]
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
    pub vd_idx: i32,

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
        let mut text_buffer = String::new();
        let mut vd = ObjVendoo::from_csv(&env.csv_path).unwrap();
        text_buffer.push_str("Vendoo lib constructed from CSV...\n");
        let mut wc = ObjWooCommerce::new_with_auth(env.wc_url, env.wc_ck, env.wc_sk);
        text_buffer.push_str("WooCommerce obj constructed with auth...\n");
        wc.fetch_populate_products().await.unwrap();
        text_buffer.push_str("WooCommerce lib constructed from HTTP...\n");

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
        let mut str = String::new();

        let rt = Runtime::new().unwrap();
        str.push_str("async ryntime created...\n");
        let shared_data = rt.block_on(SharedData::build(env));
        str.push_str("shared state constructed...\n");
        self.shared = Arc::new(Mutex::new(shared_data));
        str.push_str("shared state wrapped in Arc<Mutex<>>");
        let mut shared = self.shared.lock().unwrap();
        str.push_str("shared state locked and safely mutable...");
        str.push_str("initialized!");

        let _ = shared.text_buffer.push_str(&str);
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

impl Default for AppState {
    fn default() -> Self {
        dotenv().ok();

        let mut str = String::new();

        let wc_api_url = env::var("WC_API_URL").expect("WC_API_URL not set");
        let wc_consumer_key = env::var("WC_CONSUMER_KEY").expect("WC_CONSUMER_KEY not set");
        let wc_consumer_secret =
            env::var("WC_CONSUMER_SECRET").expect("WC_CONSUMER_SECRET not set");
        let mut csv_path: String = match env::var("CSV_PATH") {
            Ok(str) => str,
            Err(_) => String::from(state::CSV_PATH_FAILED),
        };
        let local_db = env::var("LOCAL_DB").unwrap_or(String::from("no local db!"));

        let env = BasicEnv {
            wc_url: wc_api_url.clone(),
            wc_ck: wc_consumer_key.clone(),
            wc_sk: wc_consumer_secret.clone(),

            csv_path: csv_path.clone(),
            json_path: local_db.clone(),
        };

        let rt = Runtime::new().unwrap();
        str.push_str("async ryntime created...\n");
        let shared_data = rt.block_on(SharedData::build(env.clone()));
        str.push_str("shared state constructed...\n");
        let shared = Arc::new(Mutex::new(shared_data));
        str.push_str("shared state wrapped in Arc<Mutex<>>\n");
        let mut shared_data = shared.lock().unwrap();
        str.push_str("shared state locked and safely mutable...\n");
        str.push_str("initialized!");

        shared_data.text_buffer.push_str(&str);

        // shared_data.text_buffer.clear();
        // let _ = shared_data
        //     .text_buffer
        //     .write_str("WooCommerce DB over HTTP intialized, Vendoo DB deserialized from CSV.");

        std::mem::drop(shared_data);

        Self {
            env,
            shared,
            wc_idx: 0,
            vd_idx: 0,
            initialized: true,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut shared = self.shared.lock().unwrap();
        let cpu_usage = frame.info().cpu_usage.unwrap_or(0.0);
        let cpu_usage_str = format!("CPU Usage: {}%", cpu_usage);
        let size_av = ctx.available_rect();
        let window_h = size_av.height();
        let window_w = size_av.width();

        let textbox_h = window_h * 0.6;
        let textbox_w = window_w * 0.8;

        let textbox_x = (window_w - textbox_w) / 2.0;
        let textbox_y = (window_h - textbox_h) / 2.0;

        let textbox_rect = Rect::from_min_size(
            Pos2::new(textbox_x, textbox_y),
            Vec2::new(textbox_w, textbox_h),
        );

        let lr_button_w = window_w * 0.025;
        let lr_button_h = window_h * 0.6;

        let lr_button_one_x = textbox_x - (lr_button_w) - 5.0;
        let lr_button_two_x = textbox_x + textbox_w + 5.0;
        let lr_button_y = textbox_y;

        let lr_button_left_rect = Rect::from_min_size(
            Pos2::new(lr_button_one_x, lr_button_y),
            Vec2::new(lr_button_w, lr_button_h),
        );

        let lr_button_right_rect = Rect::from_min_size(
            Pos2::new(lr_button_two_x, lr_button_y),
            Vec2::new(lr_button_w, lr_button_h),
        );

        match shared.select_mode {
            SelectMode::WC => {}
            SelectMode::VD => todo!(),
        }

        // let cpu_usage_rect = Rect::from_min_size(
        //     Pos2::new(0.0, 0.0),
        //     Vec2::new(
        //         cpu_usage_str.len() as f32,
        //         cpu_usage_str.lines().count() as f32,
        //     ),
        // );

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.disable();

            // ui.put(cpu_usage_rect, Label::new(cpu_usage_str));
            ui.label(cpu_usage_str);

            ui.put(
                // put textbox in center taking up 80%w 60%h
                textbox_rect,
                TextEdit::multiline(&mut shared.text_buffer)
                    .desired_width(textbox_w)
                    .desired_rows(20)
                    .interactive(false),
            );
            let button_lr_left = ui.put(lr_button_left_rect, Button::new("<"));
            let button_lr_right = ui.put(lr_button_right_rect, Button::new(">"));

            if button_lr_left.clicked() {
                shared.text_buffer.clear();
                shared.text_buffer.push_str("you clicked the left button!");
            }
            if button_lr_right.clicked() {
                shared.text_buffer.clear();
                shared.text_buffer.push_str("you clicked the right button!");
            }
        });
    }
}
