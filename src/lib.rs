//! # urban-rs: API for Urban Dictionary
//!
//! An API to interact with Urban Dictionary to get definitions from it.
//!
//! There are three ways to get a definition:
//!
//! * by word
//! * by id
//! * random
//!
//! ## Examples
//!
//!
//! ## License
//! [MIT](https://choosealicense.com/licenses/mit/)

use chrono::naive::NaiveDate;

/// The struct to represent an Urban definition entry.
///
#[derive(Debug, Clone)]
pub struct Definition {
    word: String,
    definition: String,
    example: String,
    author: String,
    written_on: NaiveDate,
    defid: String,
    thumbs_up: u32,
    thumbs_down: u32,
    permalink: String,
    sound_urls: Vec<String>,
}

impl PartialEq for Definition {
    fn eq(&self, other: &Self) -> bool {
        self.defid == other.defid
    }
}
