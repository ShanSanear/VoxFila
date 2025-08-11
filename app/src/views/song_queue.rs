use crate::components::QueueEntryCard;
use ::server::list_pending_queue_entries;
use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

use crate::components::SESSION_ID;

#[component]
pub fn SongQueue() -> Element {
    let queue_entries = use_resource(|| async { list_pending_queue_entries(SESSION_ID()).await });
    rsx! {
        div { class: "flex container mx-auto px-4 py-6 flex items-center justify-center flex-col",
            h1 { class: "text-2xl font-bold", "Song Queue" }
            // Here you would typically fetch and display the song queue
            // For now, we will just show a placeholder
            match &*queue_entries.read() {
                Some(Ok(entries)) => {
                    rsx! {
                        for (index , queue_entry) in entries.iter().enumerate() {
                            div { class: "flex container flex-row w-full max-w-md bg-base-100 shadow-xl my-2",
                                span { class: "text-2xl font-bold", "{index + 1}. " }
                                QueueEntryCard { queue_entry_details: queue_entry.clone() }
                                div { class: "flex flex-col gap-2",
                                    if index == 0 {
                                        button { class: "btn btn-primary", "Played" }
                                        button { class: "btn btn-secondary", "Defer" }
                                        button { class: "btn btn-error", "Remove" }
                                    }
                                }
                            }
                        }
                        if entries.is_empty() {
                            div { class: "mt-4 text-lg", "No queue entries found." }
                        }
                    }
                }
                Some(Err(e)) => {
                    debug!("Error fetching queue entries: {}", e);
                    rsx! {
                        div { class: "text-red-500", "Error fetching queue entries." }
                    }
                }
                None => {
                    rsx! {
                        div { class: "mt-4 text-lg", "Loading queue entries..." }
                    }
                }
            }
        }
    }
}
