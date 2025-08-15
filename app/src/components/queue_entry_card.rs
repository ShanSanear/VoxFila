use crate::components::{SingersCard, SongCard, SongLinksCard};
use crate::utils::{get_ising_search_link_for_song, get_yt_search_link_for_song};
use ::server::queue_entries::{move_queue_entry_after_other_entry, remove_queue_entry};
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{
    LdChevronDown, LdChevronUp, LdExternalLink, LdPause, LdPlay, LdTrash, LdYoutube,
};
use dioxus_free_icons::Icon;
use shared::models::QueueEntryDetails;

use dioxus_logger::tracing::{debug, error, info};

#[derive(PartialEq, Clone, Props)]
pub struct QueueEntryCardProps {
    index: usize,
    queue_entry_details: QueueEntryDetails,
    queue_entries_signal: Resource<Result<Vec<QueueEntryDetails>, ServerFnError>>,
    queue_entry_id_above: Option<i32>,
    queue_entry_id_below: Option<i32>,
}

#[component]
fn IconMoveUp() -> Element {
    rsx!(
        Icon { icon: LdChevronUp }
    )
}

#[component]
fn IconMoveDown() -> Element {
    rsx!(
        Icon { icon: LdChevronDown }
    )
}

#[component]
fn IconPlay() -> Element {
    rsx!(
        Icon { icon: LdPlay }
    )
}

#[component]
fn IconTrash() -> Element {
    rsx!(
        Icon { icon: LdTrash }
    )
}

#[component]
fn IconPause() -> Element {
    rsx!(
        Icon { icon: LdPause }
    )
}

#[component]
fn IconLinkWithText(link: String, text: String) -> Element {
    rsx!(
        a {
            class: "btn btn-outline btn-sm gap-2",
            href: "{link}",
            target: "_blank",
            Icon { icon: LdExternalLink }
            "{text}"
        }
    )
}

#[component]
fn IconYtLinkWithText(link: String) -> Element {
    rsx!(
        a {
            class: "btn btn-outline btn-sm gap-2",
            href: "{link}",
            target: "_blank",
            Icon { icon: LdYoutube }
            "YouTube"
        }
    )
}

#[component]
fn QueueEntryActions(
    entry_id: i32,
    queue_entry_id_below: Option<i32>,
    queue_entry_id_above: Option<i32>,
    queue_entries_signal: Resource<Result<Vec<QueueEntryDetails>, ServerFnError>>,
) -> Element {
    rsx!(
        div { class: "flex gap-1",
            button {
                class: "btn btn-outline btn-xs",
                disabled: queue_entry_id_above.is_none(),
                onclick: move |_| async move {
                    if let Some(entry_id_above) = queue_entry_id_above {
                        match move_queue_entry_after_other_entry(entry_id_above, entry_id).await {
                            Ok(_) => {
                                queue_entries_signal.restart();
                            }
                            Err(e) => {
                                error!("Error moving queue entry: {}", e);
                            }
                        }
                    }
                },
                IconMoveUp {}
            }
            button {
                class: "btn btn-outline btn-xs",
                disabled: queue_entry_id_below.is_none(),
                onclick: move |_| async move {
                    if let Some(entry_id_below) = queue_entry_id_below {
                        match move_queue_entry_after_other_entry(entry_id, entry_id_below).await {
                            Ok(_) => {
                                queue_entries_signal.restart();
                            }
                            Err(e) => {
                                error!("Error moving queue entry: {}", e);
                            }
                        }
                    }
                },
                IconMoveDown {}
            }
            button { class: "btn btn-primary btn-xs gap-1", IconPlay {} }
            button { class: "btn btn-info btn-xs gap-1", IconPause {} }
            button {
                class: "btn btn-error btn-xs gap-1",
                onclick: move |_| async move {
                    debug!("Removing queue entry with ID: {entry_id}");
                    match remove_queue_entry(entry_id).await {
                        Ok(_) => {
                            debug!("Queue entry with ID {entry_id} removed successfully.");
                            queue_entries_signal.restart();
                        }
                        Err(e) => {
                            error!("Error removing queue entry with ID {entry_id}: {e}");
                        }
                    }
                },
                IconTrash {}
            }
        }
    )
}

#[component]
pub fn QueueEntryCard(props: QueueEntryCardProps) -> Element {
    let song = props.queue_entry_details.song.clone();
    let title = song.title;
    let artist = song.artist;
    let singer_name = props.queue_entry_details.singer.name;
    let second_singer_name = props.queue_entry_details.second_singer.second_singer_name;
    let yt_link = song.yturl;
    let ising_link = song.isingurl;
    rsx!(
        div { class: "card bg-base-200 shadow-md hover:shadow-lg transition-shadow",
            div { class: "card-body",
                div { class: "flex items-start gap-3",
                    div { class: "flex flex-col items-center gap-1",
                        div { class: "badge badge-sm", "#{props.index + 1}. " }
                    }
                    div { class: "flex-1",
                        h3 { class: "card-title text-base truncate", "{title}" }
                        p { class: "text-base-content/70 truncate", "{artist}" }
                    }
                    QueueEntryActions {
                        entry_id: props.queue_entry_details.queue_entry_id,
                        queue_entries_signal: props.queue_entries_signal,
                        queue_entry_id_above: props.queue_entry_id_above,
                        queue_entry_id_below: props.queue_entry_id_below,
                    }
                }
                div { class: "space-y-2 mt-3",
                    div { class: "flex items-center gap-2 flex-wrap",
                        span { class: "text-xs", "Requested by:" }
                        div { class: "badge badge-secondary badge-sm", "{singer_name}" }
                        match second_singer_name {
                            Some(name) => rsx! {
                                div { class: "badge badge-secondary badge-sm", "{name}" }
                            },
                            None => rsx! {},
                        }
                    }
                }
                div { class: "card-actions justify-start mt-3",
                    match yt_link {
                        Some(link) => rsx! {
                            IconYtLinkWithText { link }
                        },
                        None => rsx! {
                            IconYtLinkWithText { link: get_yt_search_link_for_song(artist.as_str(), title.as_str()) }
                        },
                    }
                    match ising_link {
                        Some(link) => rsx! {
                            IconLinkWithText { link, text: "Ising" }
                        },
                        None => {
                            rsx! {
                                IconLinkWithText {
                                    link: get_ising_search_link_for_song(artist.as_str(), title.as_str()),
                                    text: "Ising",
                                }
                            }
                        }
                    }
                }
            

            }
        }
    )
}
