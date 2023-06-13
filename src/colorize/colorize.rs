use colored::*;
use regex::Regex;
use lazy_static::lazy_static;
use crate::{Change, SpecialFunction, Tag};

use super::{SFnTable, ColorizeError, parse_tags};

use std::borrow::Cow;

pub fn colorize_<'a>(
    content: &'a str,
    changes: &[Change],
    sfn_table: &SFnTable)
-> Result<Cow<'a, str>, ColorizeError> {
    if content.is_empty() { return Ok(content.into()) }
    // Find first change and convert to (Regex, &String) (from, to)
    let f_change = match changes.len() {
        0 => return Ok(content.into()),
        1 => (
            match Regex::new(changes[0].get_from()) {
                Ok(rgx) => rgx,
                Err(e) => return Err(ColorizeError::RegexPattern(e.to_string()))
            },
            changes[0].get_to()
        ),
        _ => changes
            .into_iter()
            .map(|ch| Ok((
                    match Regex::new(ch.get_from()) {
                        Ok(rgx) => rgx,
                        Err(e) => {
                            return Err(ColorizeError::RegexPattern(e.to_string()))
                        }
                    },
                    ch.get_to()
                ))
            )
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .reduce(|l, r| {
                let l_find_start = match l.0.find(content) {
                    Some(mt) => mt.start(),
                    None => return r
                };
                let r_find_start =  match r.0.find(content) {
                    Some(mt) => mt.start(),
                    None => return l
                };

                if l_find_start < r_find_start {
                    l
                } else {
                    r
                }
            })
            .unwrap()
    };
    let bounds = match f_change.0.find(content) {
        Some(mtch) => (mtch.start(), mtch.end()),
        None => return Ok(content.into())
    };

    Ok(format!(
        "{}{}{}",
        &content[..bounds.0],
        parse_tags(
            &f_change.0.replace(&content[bounds.0..bounds.1], f_change.1).to_string(),
            changes,
            &sfn_table
        )?,
        colorize_(&content[bounds.1..], changes, &sfn_table)?
    ).into())
}

pub fn colorize<'a>(
    content: &'a str,
    changes: &[Change],
    sfuncs: &[SpecialFunction])
-> Result<Cow<'a, str>, ColorizeError> {
    let sfn_table = sfuncs
        .into_iter()
        .map(|sfn| (sfn.usage().clone(), sfn.changes()))
        .collect::<SFnTable>();
    
    colorize_(content, changes, &sfn_table)
}
