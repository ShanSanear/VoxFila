use crate::components::LanguageSelector;
use crate::views::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
/// The Navbar component that will be rendered on all pages of our app
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "navbar bg-base-100 shadow-lg px-4 mx-auto py-6 max-w-7xl", // Added padding
            // App title/logo on the left
            div { class: "navbar-start", // DaisyUI navbar-start for left alignment
                Link { class: "btn btn-ghost text-xl", to: Route::SongSearch {}, "VoxFila" }
            }

            // Optional: right side for future elements like account/settings
            div { class: "navbar-end", LanguageSelector {} }
        }

        Outlet::<Route> {}
    }
}
