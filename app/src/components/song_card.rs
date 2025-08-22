use dioxus::prelude::*;
use shared::models::SongDetails;

#[derive(PartialEq, Clone, Props)]
pub struct SongCardProps {
    song: SongDetails,
}

#[component]
pub fn SongCard(props: SongCardProps) -> Element {
    rsx!(
        div { class: "flex-1",
            h2 { class: "card-title text-lg", "{props.song.title}" }
            p { class: "text-base-content/70", "{props.song.artist}" }
        }
    )
}
