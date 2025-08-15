use crate::components::{SingersCard, SongCard, SongLinksCard};
use dioxus::prelude::*;
use shared::models::QueueEntryDetails;

#[derive(PartialEq, Clone, Props)]
pub struct QueueEntryCardProps {
    index: usize,
    queue_entry_details: QueueEntryDetails,
}

#[component]
pub fn QueueEntryCard(props: QueueEntryCardProps) -> Element {
    rsx!(
        div { class: "card bg-base-200 shadow-md hover:shadow-lg transition-shadow cursor-move",
            div { class: "card-body",
                div { class: "flex items-start gap-3",
                    div { class: "flex flex-col items-center gap-1",
                        div { class: "badge badge-sm", "#{props.index + 1}. " }
                    }
                    div { class: "flex-1 min-w-0",
                        h3 {class: "card-title text-base truncate",
                        props.queue_entry_details.song.title
                    }
                        p {class: "text-base-content/70 truncate",
                        props.queue_entry_details.artist
                    }
                }
                        div {
                            class: "flex gap-1",
                            button { class: "btn btn-outline btn-xs", "Move Up"}
                            button { class: "btn btn-outline btn-xs", "Move Down"}
                            button { class: "btn btn-primary btn-xs gap-1", "Play"}
                            button { class: "btn btn-error btn-xs", "Remove"}
                        }
                    }
                div {
                    class : "space-y-2 mt-3",
                    div { class: "flex items-center gap-2 flex-wrap",
                        span { class: "text-xs", "Requested by:" }
                        div { class: "badge badge-secondary badge-sm",
                        props.queue_entry_details.singer_name
                    }
                        if props.queue_entry_details.second_singer_name.is_some() {
                            div { class: "badge badge-secondary badge-sm", props.queue_entry_details.second_singer_name}
                        }
                    }
                }
            }
        }
    )
}
