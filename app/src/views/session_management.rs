use crate::components::SessionCard;
use ::server::sessions::{create_new_session, get_current_session, list_sessions};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error};
use shared::models::NewSession;
#[component]
pub fn SessionManagement() -> Element {
    let mut current_session = use_resource(|| async move { get_current_session().await });
    rsx! {
        div {
            class: "flex container mx-auto px-4 py-6 flex items-center flex-col",
            h1 { class: "text-3xl font-bold mb-4", "Current session" }
            match &*current_session.read() {
                Some(Ok(Some(session))) => rsx! {
                   SessionCard { session: session.clone() }
                },
                Some(Err(e)) => {
                    error!("Error fetching current session id: {}", e);
                    rsx! {
                        div { class: "text-red-500", "Error fetching current session id." }
                    }
                }
                Some(Ok(None)) => rsx! {
                    div { class: "text-yellow-500", "No active session found." }
                },
                None => rsx! {
                    div { class: "text-gray-500", "Loading session id..." }
                },
            }
            button {
                class: "btn btn-primary mt-4",
                onclick: move |_| {
                    async move {
                        match create_new_session(NewSession { songs_per_singer: 1 }).await {
                            Ok(session) => {
                                debug!("Created new session: {:?}", session);
                                current_session.restart();
                            }
                            Err(e) => {
                                debug!("Error creating new session: {}", e);
                            }
                        }
                    }
                },
                "Create New Session"
            }
        }
    }
}
