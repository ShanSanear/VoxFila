use crate::components::SessionCard;
use ::server::sessions::list_sessions;
use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

#[component]
pub fn SessionManagement() -> Element {
    let sessions = use_resource(move || async move { list_sessions().await });
    rsx! {
        match &*sessions.read() {
            Some(Ok(sessions)) => {
                rsx! {
                    for session in sessions.iter() {
                        div { class: "flex container mx-auto px-4 py-6 flex items-center flex-col",
                            SessionCard { session: session.clone() }
                        }
                    }
                }
            }
            Some(Err(e)) => {
                debug!("Error fetching session details: {}", e);
                rsx! {
                    div { class: "text-red-500", "Error fetching session details." }
                }
            }
            None => {
                rsx! {
                    div { class: "text-gray-500", "Loading session details..." }
                }
            }
        }
    }
}
