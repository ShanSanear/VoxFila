use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::unic_langid::langid;

#[component]
pub fn LanguageSelector() -> Element {
    let mut i18n = i18n();

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_polish = move |_| i18n.set_language(langid!("pl-PL"));
    rsx!(
        button { class: "btn btn-ghost", onclick: change_to_english,
            label { style: "font-size: 2rem;", "🇺🇸" }
        }
        button { class: "btn btn-ghost", onclick: change_to_polish,
            label { style: "font-size: 2rem;", "🇵🇱" }
        }
    )
}

// Some components require static strings for column names or other text, hence putting this here, inside the code
pub struct LanguageStaticEnglish;

impl LanguageStaticEnglish {
    pub const SINGER_NAME_COLUMN: &'static str = "Singer";
}

pub struct LanguageStaticPolish;

impl LanguageStaticPolish {
    pub const SINGER_NAME_COLUMN: &'static str = "Wykonawca";
}
