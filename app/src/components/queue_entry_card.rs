use crate::components::{SingersCard, SongCard, SongLinksCard};
use dioxus::prelude::*;
use shared::models::QueueEntryDetails;

#[derive(PartialEq, Clone, Props)]
pub struct QueueEntryCardProps {
    queue_entry_details: QueueEntryDetails,
}

#[component]
pub fn QueueEntryCard(props: QueueEntryCardProps) -> Element {
    rsx!(
        div { class: "flex-1",
            div { class: "card-body p-0",
                SongCard { song: props.queue_entry_details.song.clone() }
                div { class: "flex gap-2 items-center",
                    SingersCard {
                        singer: props.queue_entry_details.singer.clone(),
                        second_singer: props.queue_entry_details.second_singer.clone(),
                    }
                    SongLinksCard { song: props.queue_entry_details.song.clone() }
                }
            }
        }
    )
}
