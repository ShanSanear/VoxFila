use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};
use server::{create_queue_entry, songs::get_song};
use shared::models::SongDetails;

use crate::components::SongCard;

#[derive(PartialEq, Clone, Props)]
pub struct SongRequestInputProps {
    id: i32,
}

#[component]
pub fn SongRequestInputs(props: SongRequestInputProps) -> Element {
    let mut singer_name = use_signal(|| String::new());
    let mut second_singer_name = use_signal(|| String::new());
    let mut second_singer_enabled = use_signal(|| false);
    let mut notes = use_signal(|| String::new());
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
            div { class: "flex items-center mt-2",
                label { "Add second singer" }
                input {
                    r#type: "checkbox",
                    checked: "{second_singer_enabled}",
                    class: "checkbox checkbox-primary",
                    onchange: move |e| {
                        debug!("Second singer checkbox changed: {}", e.checked());
                        second_singer_enabled.set(e.checked());
                    },
                }
            
            }
            if second_singer_enabled() {
                input {
                    r#type: "text",
                    class: "input input-bordered w-full max-w-md mt-2",
                    placeholder: "Enter other singer's name",
                    value: "{second_singer_name}",
                    oninput: move |e| {
                        debug!("Second singer name input changed: {}", e.value());
                        second_singer_name.set(e.value());
                    },
                }
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
            button {
                class: "btn btn-primary mt-4",
                onclick: move |_| {
                    async move {
                        debug!(
                            "Submitting song request with singer: {}, second singer: {}, notes: {}",
                            singer_name(), second_singer_name(), notes()
                        );
                        create_queue_entry(
                                1,
                                singer_name(),
                                props.id,
                                Some(second_singer_name()),
                                notes(),
                            )
                            .await
                            .map_err(|e| {
                                error!("Error creating queue entry: {}", e);
                            })
                            .ok();
                    }
                },
                "Submit Request"
            }
        }
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
