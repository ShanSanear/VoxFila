use crate::views::{Navbar, SongRequest, SongSearch};
use dioxus::prelude::*;
use shared::models::SongDetails;
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
        SongSearch {},
        #[route("/song-request/:id")]
        SongRequest { id: i32 }
}
