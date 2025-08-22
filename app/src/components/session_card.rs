use dioxus::prelude::*;
use shared::models::SessionDetails;

#[derive(PartialEq, Clone, Props)]
pub struct SessionCardProps {
    session: SessionDetails,
}

#[component]
pub fn SessionCard(props: SessionCardProps) -> Element {
    rsx!(
        div { class: "card w-full max-w-md bg-base-100 shadow-xl my-2",
            div { class: "card-body",
                h2 { class: "card-title", "Session with ID: {props.session.session_id} is: {props.session.state}" }
                p { class: "text-sm", "Songs per singer: {props.session.songs_per_singer}" }
                p { class: "text-sm",
                    "Current queue entry id: {props.session.current_queue_entry_id.unwrap_or(-1)}"
                }
                p { class: "text-sm",
                    r#"Last update at: {props.session.updated_at.map(|dt| dt.to_string()).unwrap_or("N/A".to_string())}"#
                }
            }
        }
    )
}
