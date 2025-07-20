use raw_window_handle::RawWindowHandle;
use mirl::platform::Buffer;

/// No access to other tabs
pub fn get_window_id_by_title(
    title: &str,
    exact_match: bool,
    case_sensitive: bool,
    include_hidden: bool,
    just_one: bool,
) -> Option<Vec<RawWindowHandle>> {
    None
}
/// No Desktop for you
pub fn capture_desktop_background_raw() -> Option<Buffer> {
    None
}

/// THIS IS POSSIBLE BUT I HAVE NO CLUE HOW
pub fn capture_screen_raw() -> Option<Buffer> {
    None
}
