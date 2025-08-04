use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info};
use server::songs::get_song;
use shared::models::SongDetails;

use crate::components::SongCard;

#[component]
pub fn SongRequest(id: i32) -> Element {
    let song = use_resource(move || async move { get_song(id).await });
    rsx! {
        match &*song.read() {
            Some(Ok(song_details)) => {
                rsx! {
                    div { class: "flex container mx-auto px-4 py-6 flex items-center justify-center flex-col",
                        h1 { "Song Request" } // TODO language
                        SongCard { song: song_details.clone() }
                    }
                }
            }
            Some(Err(e)) => {
                debug!("Error fetching song details: {}", e);
                rsx! {
                    div { class: "text-red-500", "Error fetching song details." }
                }
            }
            None => {
                rsx! {
                    div { class: "text-gray-500", "Loading song details..." }
                }
            }
        }
    }
}
