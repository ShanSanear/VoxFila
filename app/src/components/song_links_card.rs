use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;
use shared::models::SongDetails;

use crate::utils::get_yt_search_link_for_song;

#[derive(PartialEq, Clone, Props)]
pub struct SongLinksCardProps {
    song: SongDetails,
}

#[component]
pub fn SongLinksCard(props: SongLinksCardProps) -> Element {
    rsx!(
        div { class: "card w-full max-w-md bg-base-100 shadow-xl my-2",
            div { class: "card-body",
                match &props.song.yturl {
                    Some(yturl) => {
                        rsx! {
                            a { class: "link link-primary", href: "{yturl}", "YT [L]" }
                        }
                    }
                    None => {
                        rsx! {
                            a {
                                class: "link link-secondary",
                                href: get_yt_search_link_for_song(&props.song.artist, &props.song.title),
                                "YT [S]"
                            }
                        }
                    }
                }
                match &props.song.isingurl {
                    Some(isingurl) => {
                        rsx! {
                            a { class: "link link-primary", href: "{isingurl}", "YT [L]" }
                        }
                    }
                    None => {
                        rsx! {}
                    }
                }
            }
        }
    )
}
