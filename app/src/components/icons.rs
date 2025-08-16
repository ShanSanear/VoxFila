use dioxus::prelude::*;

use dioxus_free_icons::icons::ld_icons::{
    LdChevronDown, LdChevronUp, LdExternalLink, LdPause, LdPlay, LdSquarePen, LdTrash, LdYoutube,
};
use dioxus_free_icons::Icon;

#[component]
pub fn IconMoveUp() -> Element {
    rsx!(
        Icon { icon: LdChevronUp }
    )
}

#[component]
pub fn IconMoveDown() -> Element {
    rsx!(
        Icon { icon: LdChevronDown }
    )
}

#[component]
pub fn IconPlay() -> Element {
    rsx!(
        Icon { icon: LdPlay }
    )
}

#[component]
pub fn IconTrash() -> Element {
    rsx!(
        Icon { icon: LdTrash }
    )
}

#[component]
pub fn IconPause() -> Element {
    rsx!(
        Icon { icon: LdPause }
    )
}

#[component]
pub fn IconEdit() -> Element {
    rsx!(
        Icon { icon: LdSquarePen }
    )
}

#[component]
pub fn IconLinkWithText(link: String, text: String) -> Element {
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
pub fn IconYtLinkWithText(link: String) -> Element {
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
