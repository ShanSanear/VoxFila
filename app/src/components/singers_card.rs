use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;
use shared::models::{SecondSingerDetails, SingerDetails};

#[derive(PartialEq, Clone, Props)]
pub struct SongCardProps {
    singer: SingerDetails,
    second_singer: Option<SecondSingerDetails>,
}

#[component]
pub fn SingersCard(props: SongCardProps) -> Element {
    rsx!(
        div { class: "card-body",
            h2 { class: "card-title", "{props.singer.name}" }

            if let Some(second_singer) = &props.second_singer {
                h3 { class: "card-title", "{second_singer.second_singer_name}" }
            }
        }
    )
}
