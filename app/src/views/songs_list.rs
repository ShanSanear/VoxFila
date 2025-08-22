use crate::components::icons::{IconLinkWithText, IconYtLinkWithText};
use crate::components::ErrorModal;
use crate::utils::{get_ising_search_link_for_song, get_yt_search_link_for_song};
use ::server::songs::{import_songs, search_song_by_artist};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use shared::models::NewSong;

use crate::views::Route;

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

static PAGE_SIZE: usize = 15;

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

fn pagination_slots(current: usize, total: usize, window: usize) -> Vec<Option<usize>> {
    if total == 0 {
        return vec![];
    }
    let mut pages = std::collections::BTreeSet::new();
    pages.insert(1usize);
    pages.insert(total);

    let start = current.saturating_sub(window);
    let end = (current + window).min(total);

    for p in start..=end {
        if p >= 1 && p <= total {
            pages.insert(p);
        }
    }

    let mut out = Vec::new();
    let mut prev = 0usize;
    for &p in pages.iter() {
        if prev != 0 && p > prev + 1 {
            out.push(None); // ellipsis
        }
        out.push(Some(p));
        prev = p;
    }
    out
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
    let mut current_page = use_signal(|| 1usize);
    let mut total_pages = use_signal(|| 1usize);
    let mut local_frontend_search = use_signal(|| String::new());
    let matcher = SkimMatcherV2::default();
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
                                oninput: move |evt| {
                                    local_frontend_search.set(evt.value());
                                },
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
            div { class: "flex-1 overflow-y-auto overflow-x-auto",
                div { class: "",
                    match &*songs.read() {
                        Some(Ok(songs)) => {
                            let total_songs = songs.len();
                            use_effect(move || total_pages.set((total_songs + PAGE_SIZE) / PAGE_SIZE));
                            let slots = pagination_slots(current_page(), total_pages(), 2);
                            let filtered_songs = if local_frontend_search().is_empty() {
                                songs.iter().collect::<Vec<_>>()
                            } else {
                                let lowercase_frontend_search = local_frontend_search().to_lowercase();
                                let mut v: Vec<_> = songs
                                    .iter()
                                    .map(|song| {
                                        let title = song.title.to_lowercase();
                                        let artist = song.artist.to_lowercase();
                                        let text = format!("{artist} - {title}");
                                        let score = matcher
                                            .fuzzy_match(
                                                text.as_str(),
                                                lowercase_frontend_search.as_str(),
                                            )
                                            .unwrap_or(0);
                                        (song, score)
                                    })
                                    .filter(|(_, score)| *score > 0)
                                    .collect();
                                v.sort_by(|a, b| b.1.cmp(&a.1));
                                v.into_iter()
                                    .map(|(song, _)| song)
                                    .take(PAGE_SIZE * 2)
                                    .collect::<Vec<_>>()
                            };
                            let paged_songs = filtered_songs
                                .iter()
                                .skip((current_page() - 1) * PAGE_SIZE)
                                .take(PAGE_SIZE)
                                .collect::<Vec<_>>();
                            rsx! {
                                div { class: "overflow-x-auto overflow-y-auto",
                                    table { key: "songs-table-{current_tab()}", class: "table table-zebra",
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
                                
                                
                                            for (index , song) in paged_songs.iter().enumerate() {
                                                tr { key: "song-list-{index}-{current_tab()}", class: "hover",
                                                    td { class: "text-base-content/60", "{index + 1 + (current_page()-1) * PAGE_SIZE}" }
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
                                        }
                                    }
                                
                                
                                    for slot in slots.into_iter() {
                                        match slot {
                                            Some(p) => {
                                                rsx! {
                                                    button {
                                                        key: "page-button-{p}-{current_tab()}",
                                                        class: if p == current_page() { "btn btn-sm btn-primary mx-1" } else { "btn btn-sm btn-neutral mx-1" },
                                                        onclick: move |_| {
                                                            current_page.set(p);
                                                        },
                                                        "{p}"
                                                    }
                                                }
                                            }
                                            None => {
                                                rsx! {
                                                    span { class: "px-2 text-lg text-gray-500", "…" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(Err(e)) => {
                            error!("Error fetching songs: {}", e);
                            rsx! {
                                div { key: "song-list-0-{current_tab()}", class: "text-red-500", "Error fetching songs." }
                            }
                        }
                        None => rsx! {
                            div { key: "song-list-0-{current_tab()}", class: "text-gray-500", "Loading songs..." }
                        },
                    }
                }
            }
        }
    }
}
