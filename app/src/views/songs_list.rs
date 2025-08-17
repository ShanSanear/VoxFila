use crate::components::icons::{IconLinkWithText, IconYtLinkWithText};
use crate::components::ErrorModal;
use crate::utils::{get_ising_search_link_for_song, get_yt_search_link_for_song};
use ::server::songs::{import_songs, search_song_by_artist};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};
use shared::models::NewSong;

use crate::views::Route;

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[component]
pub fn TabButton(tab: String, current_tab: Signal<String>) -> Element {
    let active_tab_css_class = "tab whitespace-nowrap tab-active";
    let nonactive_tab_css_class = "tab whitespace-nowrap";
    rsx! {
        button {
            class: if tab == current_tab() { active_tab_css_class } else { nonactive_tab_css_class },
            onclick: move |_| {
                current_tab.set(tab.to_string());
            },
            "{tab}"
        }
    }
}

#[component]
pub fn ImportModal(open: Signal<bool>) -> Element {
    let mut confirmed = use_signal(|| false);
    let mut file_content = use_signal(|| String::new());
    let mut error_open = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());
    rsx! {

        dialog { class: "modal", open: "{open}",
            div { class: "modal-box",
                input {
                    r#type: "file",
                    accept: ".json",
                    class: "file-input file-input-bordered w-full max-w-xs",
                    onchange: move |evt| {
                        async move {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                let file_to_load = files.first();
                                match file_to_load {
                                    Some(file) => {
                                        info!("Processing file: {}", file);
                                        let input = file_engine.read_file_to_string(file).await;
                                        match input {
                                            Some(content) => {
                                                debug!("File content: {}", content);
                                                file_content.set(content);
                                            }
                                            None => {
                                                debug!("Input file is empty: {file}",);
                                                error_message.set("Input file is empty".to_string());
                                                error_open.set(true);
                                            }
                                        }
                                    }
                                    None => {
                                        debug!("Couldn't load file");
                                    }
                                }
                            }
                        }
                    },
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| async move {
                        let new_songs: Result<Vec<NewSong>, serde_json::Error> = serde_json::from_str(
                            file_content().as_str(),
                        );
                        match new_songs {
                            Ok(songs) => {
                                match import_songs(songs).await {
                                    Ok(_) => {
                                        info!("Successfully imported songs");
                                    }
                                    Err(e) => {
                                        error!("Failed to import songs: {}", e);
                                        error_message.set(format!("Failed to import songs: {}", e));
                                        error_open.set(true);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse JSON: {}", e);
                                error_message.set(format!("Failed to parse JSON: {}", e));
                                error_open.set(true);
                            }
                        }
                        confirmed.set(true);
                    },
                    "Submit"
                
                }
                button {
                    class: "btn btn-neutral",
                    onclick: move |_| {
                        confirmed.set(true);
                        open.set(false);
                    },
                    "OK"
                }
            }
        }
        ErrorModal { open: error_open, message: error_message() }
    }
}

#[component]
pub fn SongsList() -> Element {
    let current_tab = use_signal(|| "All songs".to_string());
    let mut open_import_modal = use_signal(|| false);
    //TODO can this be done without doing mut?
    let mut possible_tabs = vec!["All songs".to_string()];
    possible_tabs.extend(ASCII_UPPER.iter().map(|c| c.to_string()));
    //TODO need to add paging and limitation of the number of elements being returned, otherwise we could have HUGE page
    // But it might be more efficient to load ALL songs of given page in memory anyway and only then do the pagination
    let songs = use_resource(move || async move {
        let search_query = if current_tab() == "All songs" {
            "".to_string()
        } else {
            format!("{}%", current_tab())
        };
        search_song_by_artist(search_query).await
    });

    rsx! {
        ImportModal { open: open_import_modal }
        div { class: "h-full flex flex-col",
            div { class: "mb-4",
                div { class: "flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-4",
                    h2 { class: "text-2xl font-bold", "All Songs" }
                    div { class: "form-control w-full max-w-md flex items-center gap-2",
                        button {
                            class: "btn btn-info",
                            onclick: move |_| {
                                async move {
                                    open_import_modal.set(true);
                                }
                            },
                            "Import songs"
                        }
                        div { class: "relative flex-1",
                            input {
                                class: "input input-bordered w-full pl-10",
                                placeholder: "Search songs...",
                                r#type: "text",
                                value: "",
                            }
                        }
                    }
                }
                div { class: "tabs tabs-boxed overflow-x-auto flex-nowrap",
                    for tab in possible_tabs.iter() {
                        TabButton { tab, current_tab }
                    }
                }
            }
            div { class: "flex-1 overflow-hidden",
                div { class: "overflow-x-auto overflow-y-auto h-full",
                    table {
                        key: "songs-table-{current_tab()}",
                        class: "table table-zebra",
                        thead { class: "sticky top-0 bg-base-200 z-10",
                            tr {
                                th { class: "w-16", "#" }
                                th { class: "cursor-pointer hover:bg-base-300",
                                    div { class: "flex items-center gap-2", "Title" }
                                }
                                th { class: "cursor-pointer hover:bg-base-300",
                                    div { class: "flex items-center gap-2", "Artist" }
                                }
                                th { "Links" }
                            }
                        }
                        tbody { key: "songs-table-tbody-{current_tab()}",
                            match &*songs.read() {
                                Some(Ok(songs)) => rsx! {
                                    for (index , song) in songs.iter().enumerate() {
                                        tr { key: "song-list-{index}-{current_tab()}", class: "hover",
                                            td { class: "text-base-content/60", "{index + 1}" }
                                            td { class: "font-medium", "{song.title}" }
                                            td { "{song.artist}" }
                                            td {
                                                div { class: "flex gap-2",
                                                    match &song.yturl {
                                                        Some(link) => rsx! {
                                                            IconYtLinkWithText { link }
                                                        },
                                                        None => rsx! {
                                                            IconYtLinkWithText { link: get_yt_search_link_for_song(song.artist.as_str(), song.title.as_str()) }
                                                        },
                                                    }
                                                    match &song.isingurl {
                                                        Some(link) => {
                                                            rsx! {
                                                                IconLinkWithText { link, text: "Ising".to_string() }
                                                            }
                                                        }
                                                        None => rsx! {
                                                            IconLinkWithText {
                                                                link: get_ising_search_link_for_song(song.artist.as_str(), song.title.as_str()),
                                                                text: "Ising",
                                                            }
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                Some(Err(e)) => {
                                    error!("Error fetching songs: {}", e);
                                    rsx! {
                                        tr { key: "song-list-0-{current_tab()}", class: "text-red-500",
                                            td { colspan: 4, "Error fetching songs." }
                                        }
                                    }
                                }
                                None => rsx! {
                                    tr { key: "song-list-0-{current_tab()}", class: "text-gray-500",
                                        td { colspan: 4, "Loading songs..." }
                                    }
                                },
                            }
                        }
                    }
                }
            }
            div { class: "mt-4 text-center",
                div { class: "stats shadow",
                    div { class: "stat",
                        div { class: "stat-title", "Showing" }
                        div { class: "stat-value text-lg", "17" }
                        div { class: "stat-desc", "of 17 songs" }
                    }
                }
            }
        }
    }
}
