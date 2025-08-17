use crate::views::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};

#[component]
pub fn SuccessModal(
    mut open: Signal<bool>,
    title: String,
    message: String,
    redirect_target: Option<Route>,
) -> Element {
    let mut confirmed = use_signal(|| false);
    if confirmed() {
        let redirect_target = redirect_target.clone();
        match redirect_target {
            Some(target) => {
                info!("Redirecting to {:?}", target);
                let nav = navigator();
                nav.push(target);
            }
            None => open.set(false),
        }
    }
    rsx! {
        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                h2 { class: "text-lg font-bold", "{title}" }
                p { "{message}" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        confirmed.set(true);
                        open.set(false);
                    },
                    "OK"
                }
            }
        }
    }
}

#[component]
pub fn ErrorModal(open: Signal<bool>, message: String) -> Element {
    rsx! {
        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                h2 { class: "text-lg font-bold text-red-500", "Error" }
                p { class: "text-red-500", "{message}" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| open.set(false),
                    "Close"
                }
            }
        }
    }
}
