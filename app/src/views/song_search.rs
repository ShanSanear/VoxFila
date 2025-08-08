use ::server::songs::{list_songs_dummy, search_songs};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info};

use crate::components::{SongCard, UserNewSongRequest};

use crate::views::Route;

#[component]
pub fn SongSearch() -> Element {
    let mut current_search = use_signal(|| String::new());
    let songs = use_resource(move || async move {
        if current_search().len() > 3 {
            search_songs(current_search().clone()).await
        } else {
            Ok(vec![])
        }
    });
    let mut open_new_song_request = use_signal(|| false);
    rsx! {
        div { class: "flex container mx-auto px-4 py-6 flex items-center justify-center flex-col",
            input {
                id: "song-search-input",
                class: "input input-bordered w-full max-w-xl",
                placeholder: "Search for songs...",
                r#type: "search",
                oninput: move |e| {
                    debug!("Searching with query: {}", e.value());
                    current_search.set(e.value().clone());
                },
            }
            if current_search().len() < 4 {
                div { class: "mt-4",
                    p { class: "text-lg", "Please enter at least 4 characters to search." } //TODO translation
                }
            } else {
                match &*songs.read() {
                    Some(Ok(songs_list)) => {
                        rsx! {
                            for song in songs_list.iter() {
                                Link {
                                    to: Route::SongRequest {
                                        id: song.song_id,
                                    },
                                    SongCard { song: song.clone() }
                                }
                            }
                            if songs_list.is_empty() {
                                div { class: "mt-4 text-lg", "No songs found." }
                                button {
                                    class: "btn btn-primary mt-2",
                                    onclick: move |_| {
                                        debug!("Requesting adding song");
                                        open_new_song_request.set(true);
                                    },
                                    "Request adding song"
                                }
                            }
                        }
                    }
                    Some(Err(e)) => {
                        rsx! {
                            div { class: "mt-4 text-lg text-red-500", "Error fetching songs: {e}" }
                        }
                    }
                    None => {
                        rsx! {
                            div { class: "mt-4 text-lg", "Loading songs..." }
                        }
                    }
                }
            }
            UserNewSongRequest { open: open_new_song_request }
        }
    }
}
