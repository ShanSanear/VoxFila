use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;
use shared::models::SongDetails;
use table_rs::dioxus::types::TableTexts;

#[derive(PartialEq, Clone, Props)]
pub struct SongCardProps {
    song: SongDetails,
}

#[component]
pub fn SongCard(props: SongCardProps) -> Element {
    rsx!(
        div { class: "card w-full max-w-md bg-base-100 shadow-xl my-2",
            div { class: "card-body",
                h2 { class: "card-title", "{props.song.title}" }
                p { class: "text-sm", "{props.song.artist}" }
            }
        }
    )
}
