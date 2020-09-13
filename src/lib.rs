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

//std libraries
use std::fmt;

// external libraries
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

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.word, self.definition)
    }
}

/// Getter methods for a Definition
impl Definition {
    /// The word the entry is defining
    pub fn word(&self) -> &String {
        &self.word
    }

    /// The body of the definition
    pub fn definition(&self) -> &String {
        &self.definition
    }

    /// An example of the use of the word
    pub fn example(&self) -> &String {
        &self.example
    }

    /// The author of the Definition entry
    pub fn author(&self) -> &String {
        &self.author
    }

    /// The date the entry was written on
    pub fn written_on(&self) -> &NaiveDate {
        &self.written_on
    }

    /// The id of the definition
    pub fn defid(&self) -> &String {
        &self.defid
    }

    /// The number of thumbs up the entry has
    pub fn thumbs_up(&self) -> u32 {
        self.thumbs_up
    }

    /// The number of thumbs down the entry has
    pub fn thumbs_down(&self) -> u32 {
        self.thumbs_down
    }

    /// A permalink to the entry
    pub fn permalink(&self) -> &String {
        &self.permalink
    }

    /// A list of urls to sounds of the entry
    pub fn sound_urls(&self) -> &Vec<String> {
        &self.sound_urls
    }
}
