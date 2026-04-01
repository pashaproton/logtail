use regex::RegexBuilder;

use crate::error::LogtailError;

#[derive(Debug)]
pub enum Matcher {
    Always,
    Contains(String),
    ContainsInsensitive(String),
    Regex(regex::Regex),
}

#[derive(Debug)]
pub struct LineFilter {
    matcher: Matcher,
    invert: bool,
}

impl LineFilter {
    pub fn build(
        contains: Option<String>,
        regex: Option<String>,
        ignore_case: bool,
        invert: bool,
    ) -> Result<Self, LogtailError> {
        let matcher = match (contains, regex, ignore_case) {
            (Some(text), None, false) => Matcher::Contains(text),
            (Some(text), None, true) => Matcher::ContainsInsensitive(text.to_lowercase()),
            (None, Some(pattern), ignore) => {
                let compiled = RegexBuilder::new(&pattern)
                    .case_insensitive(ignore)
                    .build()?;
                Matcher::Regex(compiled)
            }
            (None, None, _) => Matcher::Always,
            (Some(_), Some(_), _) => {
                return Err(LogtailError::InvalidInput(
                    "use either --contains or --regex, not both".to_string(),
                ));
            }
        };

        Ok(Self { matcher, invert })
    }

    pub fn matches(&self, line: &str) -> bool {
        let matched = match &self.matcher {
            Matcher::Always => true,
            Matcher::Contains(text) => line.contains(text),
            Matcher::ContainsInsensitive(text) => line.to_lowercase().contains(text),
            Matcher::Regex(regex) => regex.is_match(line),
        };

        if self.invert { !matched } else { matched }
    }
}
