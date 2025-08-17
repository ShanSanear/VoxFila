use crate::views::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};

#[component]
pub fn SuccessModal(mut open: Signal<bool>, redirect_target: Option<Route>) -> Element {
    let mut confirmed = use_signal(|| false);
    if confirmed() {
        use_effect(|| match &redirect_target {
            Some(redirect_target) => {
                info!(
                    "Redirecting to {:?} after successful request.",
                    redirect_target
                );
                let nav = navigator();
                nav.push(redirect_target.clone());
            }
            _ => open.set(false),
        });
    }
    rsx! {
        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                h2 { class: "text-lg font-bold", "Song Request Submitted!" }
                p { "Your song request has been successfully submitted." }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| move {
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
pub fn InputErrorModal(open: Signal<bool>, message: String) -> Element {
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
