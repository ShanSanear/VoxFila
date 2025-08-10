pub fn get_yt_search_link_for_song(artist: &str, title: &str) -> String {
    format!("https://www.youtube.com/results?search_query={title} - {artist} karaoke",)
}
