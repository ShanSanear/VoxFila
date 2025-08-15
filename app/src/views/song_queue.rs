use crate::components::QueueEntryCard;
use ::server::list_pending_queue_entries;
use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

#[component]
pub fn SongQueue() -> Element {
    let queue_entries = use_resource(|| async { list_pending_queue_entries().await });
    rsx! {
        div { class: "flex-1 p-4 ",
            div { class: "h-[calc(100vh-12rem)] lg:h-[calc(100vh-10rem)]",
                div { class: "h-full overflow-y-auto space-y-6",
                    h2 { class: "text-xl font-bold mb-4", "Up Next" }
                    div { class: "space-y-3",
                        match &*queue_entries.read() {
                            Some(Ok(entries)) => {
                                rsx! {
                                    for (index , queue_entry) in entries.iter().enumerate() {
                                    
                                    
                                        QueueEntryCard { index, queue_entry_details: queue_entry.clone() }
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
        }
    }
}
