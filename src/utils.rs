use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

lazy_static! {
    pub static ref ID_PATTERNS: [Regex; 4] = [
        RegexBuilder::new(r"/file/d/([0-9A-Za-z_-]{10,})(?:/|$)")
            .case_insensitive(true)
            .build()
            .unwrap(),
        RegexBuilder::new(r"/folders/([0-9A-Za-z_-]{10,})(?:/|$)")
            .case_insensitive(true)
            .build()
            .unwrap(),
        RegexBuilder::new(r"id=([0-9A-Za-z_-]{10,})(?:&|$)")
            .case_insensitive(true)
            .build()
            .unwrap(),
        RegexBuilder::new(r"([0-9A-Za-z_-]{10,})")
            .case_insensitive(true)
            .build()
            .unwrap(),
    ];
    pub static ref FOLDER_PATTERN: Regex = RegexBuilder::new(
        r#"<a href="(https://drive.google.com/.*?)".*?<div class="flip-entry-title">(.*?)</div>.*?<div class="flip-entry-last-modified"><div>(.*?)</div>"#
    ).dot_matches_new_line(true).build().unwrap();

    pub static ref CONFIRM_PATTERN: Regex = RegexBuilder::new(r"download_warning[0-9A-Za-z_-]+=([0-9A-Za-z_-]+);").case_insensitive(true).build().unwrap();

    pub static ref FILENAME_PATTERN: Regex = RegexBuilder::new(r#"attachment;filename="(.*?)"#).case_insensitive(true).build().unwrap();
}

pub fn url_to_id(url: &str) -> Option<&str> {
    for pattern in ID_PATTERNS.into_iter() {
        if let Some(capture) = pattern.captures(url) {
            let match1 = capture.get(1).unwrap();
            return Some(match1.as_str());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::url_to_id;

    #[test]
    fn find_file_ids() {
        assert_eq!(
            url_to_id("https://google.com/file/d/123ga123aw1"),
            Some("123ga123aw1")
        );
    }
    #[test]
    fn find_folder_ids() {
        assert_eq!(
            url_to_id("https://google.com/folders/d/123ga123aw1"),
            Some("123ga123aw1")
        );
    }
}
