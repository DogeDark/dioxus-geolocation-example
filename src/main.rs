use dioxus::prelude::*;
use dioxus_std::{
    hooks::{self, use_geolocation},
    library::geolocation::PowerMode,
};
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    hooks::init_geolocator(cx, PowerMode::High).unwrap();
    let coords = use_geolocation(cx);

    let coords = match coords {
        Ok(v) => v,
        Err(e) => {
            let e = format!("Initializing: {:?}", e);
            return cx.render(rsx!(p { "{e}" }));
        }
    };

    render! {
        // Google maps embed
        iframe {
            width: "400",
            height: "400",
            style: "border: 1px solid black",
            src: "https://www.google.com/maps/embed/v1/view?key=AIzaSyCQY9nlxUMP7dD-W4IUOMfOUfLxzpIjna4&center={coords.latitude},{coords.longitude}&zoom=16",
        }
    }
}
