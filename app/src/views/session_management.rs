use crate::components::SessionCard;
use ::server::sessions::{create_new_session, get_current_session, list_sessions};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error};
use shared::models::NewSession;
#[component]
pub fn SessionManagement() -> Element {
    let mut sessions = use_resource(move || async move { list_sessions().await });
    let mut current_session = use_resource(|| async move { get_current_session().await });
    rsx! {
        div { class: "flex container mx-auto px-4 py-6 flex items-center flex-col",
            match &*current_session.read() {
                Some(Ok(Some(session))) => rsx! {
                    div { class: "text-green-500", "Current session id is: {session.session_id}" }
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
            match &*sessions.read() {
                Some(Ok(sessions)) => {
                    rsx! {
                        for session in sessions.iter() {
                        
                            SessionCard { session: session.clone() }
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
            button {
                class: "btn btn-primary mt-4",
                onclick: move |_| {
                    async move {
                        match create_new_session(NewSession { songs_per_singer: 1 }).await {
                            Ok(session) => {
                                debug!("Created new session: {:?}", session);
                                sessions.restart();
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
