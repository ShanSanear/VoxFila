use crate::views::{
    Navbar, SessionManagement, SongQueue, SongSelect, SongRequestForm, SongsList, TestComponent,
};
use dioxus::prelude::*;
/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        SongSelect {},
        #[route("/song-request/:id")]
        SongRequestForm { id: i32 },
        #[route("/queue")]
        SongQueue {},
        #[route("/session-management")]
        SessionManagement {},
        #[route("/songs-list")]
        SongsList {},
        #[route("/test")]
        TestComponent {}
}
