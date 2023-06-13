use crate::{Tag, Change};
use colored::*;
use regex::{Regex, Captures};
use lazy_static::lazy_static;

use std::collections::HashMap;
use std::borrow::Cow;

use super::colorize_;
use super::ColorizeError;

pub type SFnTable<'a> = HashMap<String, &'a Vec<Change>>;

lazy_static! {
    static ref TAG_PATTERN: Regex = Regex::new(r"\[%(.*?)%\]((\n|[^\n])*?)\[%/%\]").unwrap();
}

pub fn parse_tags<'a>(
    content: &'a str,
    changes: &[Change],
    sfn_table: &SFnTable,
) -> Result<Cow<'a, str>, ColorizeError> {
    let mut scope_error: Option<ColorizeError> = None;

    let res = TAG_PATTERN.replace_all(content, |capt: &Captures| {
        if scope_error.is_some() {
            return "".into()
        }

        let tag = match Tag::from_str(&capt[1]) {
            Some(t) => t,
            None => {
                scope_error = Some(
                    ColorizeError::ParseTag(
                        format!("{} not matching to any pattern", &capt[1])
                    )
                );
                return "".into()
            }
        };
        match tag {
            Tag::Style {
                color,
                bold,
                italic,
                underline
            } => {
                let cnt = 
                    capt[2]
                    .split("\n")
                    .map(|t| {
                        let mut l = t.normal();
                        if bold { l = l.bold() }
                        if italic { l = l.italic() }
                        if underline { l = l.underline() }
                        format!(
                            "{}",
                            l.truecolor(color[0], color[1], color[2])
                        )
                    })
                    .reduce(|l, r| l + "\n" + &r)
                    .unwrap_or({
                        let mut l = capt[2].normal();
                        if bold { l = l.bold() }
                        if italic { l = l.italic() }
                        if underline { l = l.underline() }
                        format!(
                            "{}",
                            l.truecolor(color[0], color[1], color[2])
                        )
                    });

                format!("{}", cnt)
            },
            Tag::Give(to) => {
                if to != "main".to_string() {
                    match colorize_(&capt[2], &sfn_table[&to], sfn_table) {
                        Ok(s) => s.to_string(),
                        Err(e) => { scope_error = Some(e); "".to_string() }
                    }
                } else {
                    match colorize_(&capt[2], changes, sfn_table) {
                        Ok(s) => s.to_string(),
                        Err(e) => { scope_error = Some(e); "".to_string() }
                    }
                }
            }
        }
    });

    if scope_error.is_some() {
        return Err(scope_error.unwrap())
    };

    Ok(res)
}
