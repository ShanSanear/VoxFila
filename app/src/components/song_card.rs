use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;
use shared::models::SongDetails;

#[derive(PartialEq, Clone, Props)]
pub struct SongCardProps {
    song: SongDetails,
}

#[component]
pub fn SongCard(props: SongCardProps) -> Element {
    rsx!(

            h2 { class: "card-title mb-1", "{props.song.title}" }
            p { class: "text-sm text-base-content mb-1", "{props.song.artist}" }

    )
}
