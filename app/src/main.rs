mod components;
mod utils;
mod views;

use crate::views::Route;

use dioxus::{
    logger::{self, tracing},
    prelude::*,
};
use dioxus_i18n::{
    prelude::{use_init_i18n, I18nConfig},
    unic_langid::langid,
};

const FAVICON: Asset = asset!("assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("pl-PL"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("pl-PL"), include_str!("./i18n/pl-PL.ftl")))
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "VoxFila" }
        main { class: "min-h-screen bg-base-200", Router::<Route> {} }
    }
}
