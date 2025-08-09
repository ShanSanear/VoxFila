use ::server::songs::save_song;
use dioxus::html::u;

use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;
use dioxus_logger::tracing::{debug, error, info};
use shared::models::NewSong;
use shared::models::SongDetails;

use crate::views::Route;

#[component]
pub fn UserNewSongRequestDialogSuccess(
    open: Signal<bool>,
    parent_open: Signal<bool>,
    song_id: Option<i32>,
) -> Element {
    let mut confirmed = use_signal(|| false);
    let mut route_target: Signal<Option<Route>> = use_signal(|| None);
    if confirmed() {
        use_effect(move || {
            info!("Redirecting to song search after successful request.");
            let nav = navigator();
            if let Some(route) = route_target() {
                nav.push(route);
            } else {
                error!("No route target set after confirmation.");
            }
        });
    }
    rsx! {
        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                h2 { class: "text-lg font-bold", "Song Request Submitted!" }
                p { "Your song request has been successfully submitted." }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        confirmed.set(true);
                        open.set(false);
                        parent_open.set(false);
                        route_target.set(Some(Route::SongSearch {}));
                    },
                    "Go to Song Search"
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        confirmed.set(true);
                        open.set(false);
                        parent_open.set(false);
                        match song_id {
                            Some(song_id) => {
                                info!("Redirecting to song request with ID: {}", song_id);
                                route_target.set(Some(Route::SongRequest { id: song_id }));
                            }
                            None => {
                                error!("No song ID provided!");
                            }
                        }
                    },
                    "Request that song"
                
                }
            }
        }
    }
}

#[component]
pub fn UserNewSongRequest(open: Signal<bool>) -> Element {
    let mut confirmed = use_signal(|| false);
    let mut open_dialog = use_signal(|| false);
    let mut song_id: Signal<Option<i32>> = use_signal(|| None);
    let mut song_title = use_signal(|| String::new());
    let mut song_artist = use_signal(|| String::new());
    let mut yturl: Signal<Option<String>> = use_signal(|| None);
    let mut isingurl: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                h2 { class: "text-lg font-bold", "Song Request Submitted!" }
                input {
                    class: "input input-bordered w-full max-w-xs",
                    placeholder: "Enter song name",
                    r#type: "text",
                    oninput: move |e| {
                        song_title.set(e.value());
                    },
                }
                input {
                    class: "input input-bordered w-full max-w-xs mt-2",
                    placeholder: "Enter song artist",
                    r#type: "text",
                    oninput: move |e| {
                        song_artist.set(e.value());
                    },
                }
                input {
                    class: "input input-bordered w-full max-w-xs mt-2",
                    placeholder: "YouTube URL (optional)",
                    r#type: "url",
                    oninput: move |e| {
                        if e.value().is_empty() {
                            yturl.set(None);
                        } else {
                            yturl.set(Some(e.value()));
                        }
                    },
                }
                input {
                    class: "input input-bordered w-full max-w-xs mt-2",
                    placeholder: "iSing URL (optional)",
                    r#type: "url",
                    oninput: move |e| {
                        if e.value().is_empty() {
                            isingurl.set(None);
                        } else {
                            isingurl.set(Some(e.value()));
                        }
                    },
                }
                button {
                    class: "btn btn-primary mt-4",
                    onclick: move |_| {
                        async move {
                            let new_song = NewSong {
                                artist: song_artist(),
                                title: song_title(),
                                yturl: yturl(),
                                isingurl: isingurl(),
                            };
                            if !new_song.validate() {
                                error!("Invalid song request data.");
                                open_dialog.set(false);
                            } else {
                                match save_song(new_song).await {
                                    Ok(song_details) => {
                                        song_id.set(Some(song_details.song_id));
                                        open_dialog.set(true);
                                    }
                                    Err(e) => {
                                        error!("Error saving song: {}", e);
                                    }
                                };
                            }
                        }
                    },
                    "Submit"
                }
                UserNewSongRequestDialogSuccess {
                    open: open_dialog,
                    parent_open: open,
                    song_id: song_id(),
                }
            }
        }
    }
}
