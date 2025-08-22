use crate::components::LanguageSelector;
use crate::views::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "navbar bg-base-100 shadow-lg px-4 mx-auto py-6 max-w-7xl",
            div { class: "navbar-start",
                Link { class: "btn btn-ghost text-xl", to: Route::SongSelect {}, "VoxFila" }
            }
            div { class: "navbar-center",
                Link { class: "btn btn-ghost", to: Route::SongSelect {}, "Select song" }
            }
            div { class: "navbar-center",
                Link { class: "btn btn-ghost", to: Route::SongQueue {}, "Queue" }
            }
            div { class: "navbar-center",
                Link { class: "btn btn-ghost", to: Route::SessionManagement {}, "Session Management" }
            }
            div { class: "navbar-center",
                Link { class: "btn btn-ghost", to: Route::SongsList {}, "Songs List" }
            }

            div { class: "navbar-center",
                Link { class: "btn btn-ghost", to: Route::TestComponent {},
                    {}
                    "Test"
                }
            }

            div { class: "navbar-end", LanguageSelector {} }
        }
        // Main container of the app
        div { class: "flex container h-[calc(100vh-12rem)] lg:h-[calc(100vh-10rem)] min-w-80 md:shrink-0 mx-auto px-4 py-6 items-center flex-col flex-4/5",
            Outlet::<Route> {}
        }
    }
}
