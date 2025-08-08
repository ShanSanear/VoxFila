use dioxus::prelude::*;
pub static SESSION_ID: GlobalSignal<i32> = Global::new(|| 1);
