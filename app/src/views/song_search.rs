use dioxus::prelude::*;

#[component]
pub fn SongSearch() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-6", "This is my search song object" }
    }
}
