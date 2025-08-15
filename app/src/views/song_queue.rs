use crate::components::QueueEntryCard;
use ::server::list_pending_queue_entries;
use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

#[component]
pub fn SongQueue() -> Element {
    let mut queue_entries = use_resource(|| async { list_pending_queue_entries().await });
    rsx! {
        div { class: "flex-1 p-4 ",
            div { class: "h-[calc(100vh-12rem)] lg:h-[calc(100vh-10rem)]",
                div { class: "h-full overflow-y-auto space-y-6",
                    h2 { class: "text-xl font-bold mb-4", "Up Next" }
                    div { class: "space-y-3",
                        match &*queue_entries.read() {
                            Some(Ok(entries)) => {
                                let decorated: Vec<(usize, _, Option<i32>, Option<i32>)> = entries
                                    .iter()
                                    .enumerate()
                                    .map(|(index, entry)| {
                                        let above = index
                                            .checked_sub(1)
                                            .and_then(|idx| entries.get(idx))
                                            .map(|e| e.queue_entry_id);
                                        let below = entries.get(index + 1).map(|e| e.queue_entry_id);
                                        (index, entry.clone(), above, below)
                                    })
                                    .collect();
                                rsx! {
                                    for (index , queue_entry , queue_entry_id_above , queue_entry_id_below) in decorated.into_iter() {
                                        QueueEntryCard {
                                            index,
                                            queue_entry_details: queue_entry,
                                            queue_entries_signal: queue_entries,
                                            queue_entry_id_above,
                                            queue_entry_id_below,
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
        }
    }
}
