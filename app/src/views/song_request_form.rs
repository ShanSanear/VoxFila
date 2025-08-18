use crate::components::SongCard;
use crate::views::Route;
use ::server::{create_queue_entry, sessions::get_current_session, songs::get_song};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};

use crate::components::{ErrorModal, SuccessModal};

use shared::utils::validation::validate_inputs;

use dioxus_free_icons::icons::ld_icons::{LdStickyNote, LdUser};
use dioxus_free_icons::Icon;

#[derive(PartialEq, Clone, Props)]
pub struct SongRequestFormInputsProps {
    id: i32,
}

#[component]
pub fn SongRequestFormInputs(props: SongRequestFormInputsProps) -> Element {
    let mut singer_name = use_signal(|| String::new());
    let mut second_singer_name: Signal<Option<String>> = use_signal(|| None);
    let mut notes = use_signal(|| String::new());
    let mut open_success = use_signal(|| false);
    let mut error_open = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());

    //TODO translations
    rsx! {
        div { class: "space-y-4",
            div { class: "form-control",

                label { class: "label",
                    Icon { icon: LdUser {} }
                    span { class: "label-text", "Your Name *" }
                }
                input {
                    r#type: "text",
                    class: "input input-bordered",
                    placeholder: "Enter your name",
                    value: "{singer_name}",
                    oninput: move |e| {
                        debug!("Singer name input changed: {}", e.value());
                        singer_name.set(e.value());
                    },
                }
                div { class: "form-control",
                    label { class: "label",
                        Icon { icon: LdUser {} }
                        span { class: "label-text flex items-center gap-2", "Duet Partner (Optional)" }
                    }
                    input {
                        r#type: "text",
                        class: "input input-bordered",
                        placeholder: "Name of duet partner",
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
                }

                div { class: "form-control",
                    label { class: "label",
                        Icon { icon: LdStickyNote {} }
                        span { class: "label-text", "Notes (Optional)" }
                    }
                    textarea {
                        class: "input input-bordered",
                        placeholder: "Any special notes or requests...",
                        value: "{notes}",
                        rows: "3",
                        oninput: move |e| {
                            debug!("Notes input changed: {}", e.value());
                            notes.set(e.value());
                        },
                    }
                }
                div { class: "modal-action",
                    button {
                        class: "btn btn-primary",
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
                        "Add to Queue"
                    }
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| {
                            let nav = navigator();
                            nav.push(Route::SongSelect {});
                        },
                        "Cancel"
                    }
                }
            }
            SuccessModal {
                open: open_success,
                title: "Song Request Submitted!".to_string(),
                message: "Your song request has been successfully submitted.".to_string(),
                redirect_target: Some(Route::SongSelect {}),
            }
            ErrorModal { open: error_open, message: error_message() }
        }
    }
}

#[component]
pub fn SongRequestForm(id: i32) -> Element {
    let song = use_resource(move || async move { get_song(id).await });
    rsx! {
        match &*song.read() {
            Some(Ok(song_details)) => {
                rsx! {
                    div { class: "flex container mx-auto px-4 py-6 flex items-center flex-col",
                        h3 { class: "font-bold text-lg mb-4", "Request Song" }
                        p { class: "text-sm opacity-70 mb-4",
                            "Fill out the form below to add this song to the karaoke queue."
                        }
                        SongCard { song: song_details.clone() }
                        SongRequestFormInputs { id }
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
