use dioxus::prelude::*;
use shared::models::{SecondSingerDetails, SingerDetails};

#[derive(PartialEq, Clone, Props)]
pub struct SingerCardProps {
    singer: SingerDetails,
    second_singer: Option<SecondSingerDetails>,
}

#[component]
pub fn SingersCard(props: SingerCardProps) -> Element {
    rsx!(
        span { class: "badge badge-info", "{props.singer.name}" }
        match &props.second_singer {
            Some(second_singer) => {
                match &second_singer.second_singer_name {
                    Some(second_singer_name) => {
                        if !second_singer_name.is_empty() {
                            rsx! {
                                span { class: "badge badge-info", "{second_singer_name}" }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                    None => rsx! {},
                }
            }
            &None => rsx! {},
        }
    )
}
