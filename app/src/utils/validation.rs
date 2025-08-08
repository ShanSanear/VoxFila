use dioxus_logger::tracing::error;

pub fn input_has_valid_characters(input_string: &str) -> bool {
    input_string
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace())
}

pub fn validate_inputs(
    singer_name: &str,
    second_singer_name: &Option<String>,
    notes: &str,
) -> bool {
    if singer_name.is_empty() {
        error!("Singer name cannot be empty.");
        return false;
    }
    if !input_has_valid_characters(singer_name) {
        error!("Singer name is invalid: {}", singer_name);
        return false;
    }
    match second_singer_name {
        Some(name) if !name.is_empty() => {
            if !input_has_valid_characters(name) {
                error!("Second singer name is invalid: {}", name);
                return false;
            }
        }
        _ => {}
    }

    if !input_has_valid_characters(notes) {
        error!(
            "Notes must contain only alphanumeric characters. Notes: {}",
            notes
        );
        return false;
    }

    true
}
