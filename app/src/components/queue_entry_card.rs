use crate::components::{SingersCard, SongCard};
use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;
use shared::models::QueueEntryDetails;

#[derive(PartialEq, Clone, Props)]
pub struct QueueEntryCardProps {
    queue_entry_details: QueueEntryDetails,
}

#[component]
pub fn QueueEntryCard(props: QueueEntryCardProps) -> Element {
    rsx!(
        div { class: "card flex container flex-row w-full max-w-md bg-base-100 shadow-xl my-2",
            SongCard { song: props.queue_entry_details.song.clone() }
            SingersCard {
                singer: props.queue_entry_details.singer.clone(),
                second_singer: props.queue_entry_details.second_singer.clone(),
            }
        }
    )
}
