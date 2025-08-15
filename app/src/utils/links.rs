use url::form_urlencoded;
pub fn get_yt_search_link_for_song(artist: &str, title: &str) -> String {
    format!("https://www.youtube.com/results?search_query={title} - {artist} karaoke",)
}

pub fn get_ising_search_link_for_song(artist: &str, title: &str) -> String {
    let input = format!("{title} - {artist}",);
    let encoded: String = form_urlencoded::byte_serialize(input.as_bytes()).collect();
    format!("https://ising.pl/search?q={encoded}")
}
