use dioxus::prelude::*;
use dioxus_std::{
    hooks::{self, use_geolocation},
    library::geolocation::PowerMode,
};
use log::LevelFilter;
use wasm_bindgen::prelude::*;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    initMap();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    hooks::init_geolocator(cx, PowerMode::High).unwrap();
    let coords = use_geolocation(cx);

    match coords {
        Ok(v) => {
            panTo(v.latitude, v.longitude);
        },
        Err(e) => {
            let e = format!("Initializing: {:?}", e);
            return cx.render(rsx!(p { "{e}" }));
        }
    };

    None
}

#[wasm_bindgen(module = "/src/map.js")]
extern "C" {
    pub fn initMap();
    pub fn panTo(lat: f64, lon: f64);
}
