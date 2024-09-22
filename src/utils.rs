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
    LP,
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
    pub local_session: Option<LocalSession>,

    pub text_buffer: String,
    pub select_mode: SelectMode,
    pub local_init: bool,
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
        let local_init: bool = false;

        Self {
            vd: Some(vd),
            wc: Some(wc),
            local_session: None,

            text_buffer,
            select_mode,
            local_init,
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

// TODO LOTS OF THINGS

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

        let switch_mode_wc_button_w = textbox_w / 3.0 + (lr_button_w / 2.0);
        let swtich_mode_wc_button_h = window_h * 0.025;

        let switch_mode_wc_button_x = textbox_x - lr_button_w - 5.0;
        let switch_mode_wc_button_y = textbox_y + textbox_h + 5.0;

        let switch_mode_wc_button_rect = Rect::from_min_size(
            Pos2::new(switch_mode_wc_button_x, switch_mode_wc_button_y),
            Vec2::new(switch_mode_wc_button_w, swtich_mode_wc_button_h),
        );

        let switch_mode_vd_button_w = textbox_w / 3.0;
        let switch_mode_vd_button_h = window_h * 0.025;

        let switch_mode_vd_button_x = textbox_x + (textbox_w / 3.0);
        // switch_mode_vd_button_y = switch_mode_wc_button_y;

        let switch_mode_vd_button_rect = Rect::from_min_size(
            Pos2::new(switch_mode_vd_button_x, switch_mode_wc_button_y),
            Vec2::new(switch_mode_vd_button_w, switch_mode_vd_button_h),
        );

        let switch_mode_lp_button_w = switch_mode_wc_button_w;
        let switch_mode_lp_button_h = window_h * 0.025;

        let switch_mode_lp_button_x =
            textbox_x + (2.0 * (textbox_w / 3.0)) + ((1.0 / 2.0) * lr_button_w) + 5.0;
        // y same

        let switch_mode_lp_button_rect = Rect::from_min_size(
            Pos2::new(switch_mode_lp_button_x, switch_mode_wc_button_y),
            Vec2::new(switch_mode_lp_button_w, switch_mode_lp_button_h),
        );

        // match shared.select_mode {
        //     SelectMode::WC => {}
        //     SelectMode::VD => todo!(),
        // }

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

            let _ = ui.put(
                // put textbox in center taking up 80%w 60%h
                textbox_rect,
                TextEdit::multiline(&mut shared.text_buffer)
                    .desired_width(textbox_w)
                    .desired_rows(20)
                    .interactive(false),
            );
            let button_lr_left = ui.put(lr_button_left_rect, Button::new("<"));
            let button_lr_right = ui.put(lr_button_right_rect, Button::new(">"));

            let switch_mode_wc_button =
                ui.put(switch_mode_wc_button_rect, Button::new("TO WOOCOMM MODE"));

            if switch_mode_wc_button.clicked() {
                shared.select_mode = SelectMode::WC;
                shared.text_buffer.clear();
                // shared.text_buffer.push_str(string);
            }

            let switch_mode_vd_button =
                ui.put(switch_mode_vd_button_rect, Button::new("TO VENDOO MODE"));

            if switch_mode_vd_button.clicked() {
                shared.select_mode = SelectMode::VD;
                shared.text_buffer.clear();
            }

            let switch_mode_lp_button = ui.put(
                switch_mode_lp_button_rect,
                Button::new("SWITCH TO LP MODE (SERIALIZES ALL PRODUCTS)"),
            );

            match shared.select_mode {
                SelectMode::WC => {
                    let wc_button_w = window_w * 0.25;
                    let wc_button_h = window_h * 0.025;
                    let wc_button_x = (window_w / 2.0) - (wc_button_w / 2.0);
                    let wc_button_y = textbox_y * 0.5;
                    let wc_button_rect = Rect::from_min_size(
                        Pos2::new(wc_button_x, wc_button_y),
                        Vec2::new(wc_button_w, wc_button_h),
                    );

                    let wc_label_button = ui.put(wc_button_rect, Button::new("WOOCOMMERCE MODE."));

                    if wc_label_button.clicked() {
                        // do nothing
                    }
                }
                SelectMode::VD => {
                    let vd_button_w = window_w * 0.25;
                    let vd_button_h = window_h * 0.025;
                    let vd_button_x = (window_w / 2.0) - (vd_button_w / 2.0);
                    let vd_button_y = textbox_y * 0.5;
                    let vd_button_rect = Rect::from_min_size(
                        Pos2::new(vd_button_x, vd_button_y),
                        Vec2::new(vd_button_w, vd_button_h),
                    );

                    let vd_label_button = ui.put(vd_button_rect, Button::new("WOOCOMMERCE MODE."));

                    if vd_label_button.clicked() {
                        // do nothing, this button is meaningless.
                    }
                }
                SelectMode::LP => {
                    if !shared.local_init {
                        shared.text_buffer.clear();
                        shared
                            .text_buffer
                            .push_str("LocalSession has not yet been configured!");

                        shared.select_mode = SelectMode::WC;
                    } else {
                        // do stuff.
                    }
                }
            }

            if switch_mode_lp_button.clicked() {
                shared.select_mode = SelectMode::LP;
                shared.text_buffer.clear();
                shared
                    .text_buffer
                    .push_str("SERIALIZING ALL PRODUCTS INTO LocalSession...\n");

                if !shared.local_init {
                    shared.local_session = Some(crate::local::LocalSession::from_session(
                        shared.wc.clone().unwrap(),
                        shared.vd.clone().unwrap(),
                    ));
                    shared
                        .text_buffer
                        .push_str("PRODUCTS SERIALIZED INTO LOCALSESSION!");
                }
            }

            if button_lr_left.clicked() {
                shared.text_buffer.clear();
                shared.text_buffer.push_str("you clicked the left button!");

                match shared.select_mode {
                    SelectMode::WC => {
                        if self.wc_idx <= 0 {
                            self.wc_idx = 0;
                        } else {
                            self.wc_idx -= 1;
                        }
                        let prod =
                            <std::option::Option<ObjWooCommerce> as Clone>::clone(&shared.wc)
                                .unwrap()
                                .products
                                .unwrap()
                                .get(self.wc_idx as usize)
                                .unwrap_or(
                                    &shared.wc.clone().unwrap().products.unwrap().get(0).unwrap(),
                                )
                                .clone();

                        let str = prod.debug();
                        shared.text_buffer.clear();
                        shared.text_buffer.push_str(&str);
                    }
                    SelectMode::VD => {
                        if self.vd_idx <= 0 {
                            self.vd_idx = 0;
                        } else {
                            self.vd_idx -= 1;
                        }

                        let prod = shared
                            .vd
                            .as_ref()
                            .unwrap()
                            .products
                            .as_ref()
                            .unwrap()
                            .get(self.vd_idx as usize)
                            .unwrap();

                        let str = prod.debug();
                        shared.text_buffer.clear();
                        shared.text_buffer.push_str(&str);
                    }
                    SelectMode::LP => {
                        // TODO!
                    }
                }
            }
            if button_lr_right.clicked() {
                shared.text_buffer.clear();
                shared.text_buffer.push_str("you clicked the right button!");

                match shared.select_mode {
                    SelectMode::WC => {
                        let len = shared.wc.as_ref().unwrap().get_length();

                        if self.wc_idx >= len {
                            self.wc_idx = len
                        } else {
                            self.wc_idx += 1;
                        }

                        let prod =
                            <std::option::Option<ObjWooCommerce> as Clone>::clone(&shared.wc)
                                .unwrap()
                                .products
                                .unwrap()
                                .get(self.wc_idx as usize)
                                .unwrap_or(
                                    &shared
                                        .wc
                                        .clone()
                                        .unwrap()
                                        .products
                                        .unwrap()
                                        .get(len as usize - 1)
                                        .unwrap(),
                                )
                                .clone();

                        let str = prod.debug();
                        shared.text_buffer.clear();
                        shared.text_buffer.push_str(&str);
                    }
                    SelectMode::VD => {
                        let len = shared.vd.as_ref().unwrap().get_length();

                        if self.vd_idx >= len {
                            self.vd_idx = len
                        } else {
                            self.vd_idx += 1;
                        }

                        let prod = shared
                            .vd
                            .as_ref()
                            .unwrap()
                            .products
                            .as_ref()
                            .unwrap()
                            .get(self.vd_idx as usize)
                            .unwrap();

                        let str = prod.debug();
                        shared.text_buffer.clear();
                        shared.text_buffer.push_str(&str);
                    }
                    SelectMode::LP => {
                        let len = shared.local_session.as_ref().unwrap().local_wp.len() as i32;
                        if self.vd_idx >= len {
                            self.vd_idx = len
                        } else {
                            self.vd_idx += 1;
                        }

                        // if let Some(local_session) = shared.local_session {
                        //     local_session
                        // } else {
                        //     shared.select_mode = SelectMode::WC;
                        // }
                    }
                }
            }
        });
    }
}
