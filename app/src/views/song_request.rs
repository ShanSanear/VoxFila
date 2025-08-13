use crate::components::SongCard;
use crate::views::Route;
use ::server::{create_queue_entry, sessions::get_current_session, songs::get_song};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};

use shared::utils::validation::validate_inputs;

#[derive(PartialEq, Clone, Props)]
pub struct SongRequestInputProps {
    id: i32,
}

#[component]
pub fn SuccessModal(open: Signal<bool>) -> Element {
    let mut confirmed = use_signal(|| false);
    if confirmed() {
        use_effect(|| {
            info!("Redirecting to song search after successful request.");
            let nav = navigator();
            nav.push(Route::SongSearch {});
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
                    },
                    "OK"
                }
            }
        }
    }
}

#[component]
pub fn InputErrorModal(open: Signal<bool>, message: String) -> Element {
    rsx! {
        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                h2 { class: "text-lg font-bold text-red-500", "Error" }
                p { class: "text-red-500", "{message}" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| open.set(false),
                    "Close"
                }
            }
        }
    }
}

#[component]
pub fn SongRequestInputs(props: SongRequestInputProps) -> Element {
    let mut singer_name = use_signal(|| String::new());
    let mut second_singer_name: Signal<Option<String>> = use_signal(|| None);
    let mut notes = use_signal(|| String::new());
    let mut open_success = use_signal(|| false);
    let mut error_open = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());

    //TODO translations
    rsx! {
        div { class: "flex flex-col items-center w-full max-w-md",
            input {
                r#type: "text",
                class: "input input-bordered w-full max-w-md mt-2",
                placeholder: "Enter your name",
                value: "{singer_name}",
                oninput: move |e| {
                    debug!("Singer name input changed: {}", e.value());
                    singer_name.set(e.value());
                },
            }
            input {
                r#type: "text",
                class: "input input-bordered w-full max-w-md mt-2",
                placeholder: "Enter other singer's name (Optional)",
                value: "{second_singer_name().unwrap_or_default()}",
                oninput: move |e| {
                    debug!("Second singer name input changed: {}", e.value());
                    if e.value().is_empty() {
                        second_singer_name.set(None);
                    } else {
                        second_singer_name.set(Some(e.value()));
                    }
                },
            }


            input {
                r#type: "text",
                class: "input input-bordered w-full max-w-md mt-2 h-20",
                placeholder: "Enter any notes",
                value: "{notes}",
                oninput: move |e| {
                    debug!("Notes input changed: {}", e.value());
                    notes.set(e.value());
                },
            }
            div { class: "mt-2 flex items-center justify-between w-full",
                button {
                    class: "btn btn-primary mx-4",
                    onclick: move |_| {
                        async move {
                            debug!(
                                "Submitting song request with singer: {}, second singer: {}, notes: {}",
                                singer_name(), second_singer_name().unwrap_or_default(), notes()
                            );
                            if !validate_inputs(&singer_name(), &second_singer_name(), &notes()) {
                                error!("Invalid inputs provided for song request.");
                                error_open.set(true);
                            } else {
                                match create_queue_entry(
                                        singer_name(),
                                        props.id,
                                        second_singer_name(),
                                        notes(),
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        debug!("Song request created successfully.");
                                        open_success.set(true);
                                    }
                                    Err(e) => {
                                        error!("Error creating queue entry: {e}");
                                        error_message.set(format!("Error creating queue entry: {e}"));
                                        error_open.set(true);
                                    }
                                }
                            }
                        }
                    },
                    "Submit Request"
                }
                button {
                    class: "btn btn-secondary mx-4",
                    onclick: move |_| {
                        let nav = navigator();
                        nav.push(Route::SongSearch {});
                    },
                    "Back to Search"
                }
            }
        }
        SuccessModal { open: open_success }
        InputErrorModal { open: error_open, message: error_message() }
    }
}

#[component]
pub fn SongRequest(id: i32) -> Element {
    let song = use_resource(move || async move { get_song(id).await });
    rsx! {
        match &*song.read() {
            Some(Ok(song_details)) => {
                rsx! {
                    div { class: "flex container mx-auto px-4 py-6 flex items-center flex-col",
                        SongCard { song: song_details.clone() }
                        SongRequestInputs { id }
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
