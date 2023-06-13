use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATTERN_STYLE: Regex = Regex::new(r"^#([[:xdigit:]]{6}) ([01]{3})$").unwrap();
    static ref PATTERN_GIVE: Regex = Regex::new(r"^[[:word:]]+$").unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Style {
        color: [u8; 3],
        bold: bool,
        italic: bool,
        underline: bool
    },
    Give(String)
}

impl Tag {
    pub fn from_str(content: &str) -> Option<Self> {
        // [%sfunc_title%]...[%/%]
        // [%#AAAAAA 000%]...[%/%]
        // |------------|
        //    content
        if let Some(capt) = PATTERN_STYLE.captures(content) {
            let color = [
                u8::from_str_radix(&capt[1][..2], 16).unwrap(),
                u8::from_str_radix(&capt[1][2..4], 16).unwrap(),
                u8::from_str_radix(&capt[1][4..6], 16).unwrap()
            ];
            let (bold, italic, underline) = (
                &(&capt[2])[0..1] == "1",
                &(&capt[2])[1..2] == "1",
                &(&capt[2])[2..3] == "1",
            );

            Some(Tag::Style {
                color,
                bold,
                italic,
                underline
            })
        } else if let Some(capt) = PATTERN_GIVE.captures(content) {
            Some(Tag::Give(capt[0].to_string()))
        } else { None }
    }
}
