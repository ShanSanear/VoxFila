use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info};

use server::songs::{list_songs_dummy, search_songs};

#[component]
pub fn SongSearch() -> Element {
    let mut current_search = use_signal(|| String::new());
    let songs = use_resource(move || async move {
        if current_search().len() > 3 {
            // search_songs(current_search().clone()).await
            list_songs_dummy().await
        } else {
            Ok(vec![])
        }
    });
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
                                div { class: "card w-full max-w-md bg-base-100 shadow-xl my-2",
                                    div { class: "card-body",
                                        h2 { class: "card-title", "{song.title}" }
                                        p { class: "text-sm", "{song.artist}" }
                                    }
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
        }
    }
}
