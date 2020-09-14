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
use std::{fmt, convert};

// external libraries
use chrono::naive::NaiveDate;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Defid(u64);

impl Defid {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// The struct to represent an Urban definition entry.
#[derive(Debug, Clone)]
pub struct Definition {
    word: String,
    definition: String,
    example: String,
    author: String,
    written_on: NaiveDate,
    defid: Defid,
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
    fn new(json_definition: &serde_json::Value) -> Option<Definition> {

        let word = json_definition["word"].as_str()?.to_string();
        let definition = json_definition["definition"].as_str()?.to_string();
        let example = json_definition["example"].as_str()?.to_string();
        let author = json_definition["author"].as_str()?.to_string();
        let parsed_date_str = json_definition["written_on"].as_str()?;
        let written_on = NaiveDate::parse_from_str(
            parsed_date_str,
            "%Y-%m-%dT%H:%M:%S%.3fZ"
        ).ok()?;
        let defid = Defid(json_definition["defid"].as_u64()?);
        let thumbs_up = json_definition["thumbs_up"].as_u64()? as u32;
        let thumbs_down = json_definition["thumbs_down"].as_u64()? as u32;
        let permalink = json_definition["permalink"].as_str()?.to_string();
        let sound_urls = json_definition["sound_urls"].as_array()?
            .iter().filter_map(|j_url| j_url.as_str())
            .map(|s_url| s_url.to_string()).collect();

        Some(Definition {
            word,
            definition,
            example,
            author,
            written_on,
            defid,
            thumbs_up,
            thumbs_down,
            permalink,
            sound_urls,
        })
    }

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
    pub fn defid(&self) -> Defid {
        self.defid
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

// API Functions

/// Get a list of definitions trough a reqwest client by word.
pub async fn define(client: &reqwest::Client, word: &str) -> Result<Vec<Definition>, UrbanError> {
    let response = client.get(&format!("https://api.urbandictionary.com/v0/define?term={}", word))
        .send()
        .await?
        .text()
        .await?;

    //json list of all the definitions.
    let json_definitions: serde_json::Value = serde_json::from_str::<serde_json::Value>(&response)?;

    Ok(json_definitions.get("list")
        .ok_or_else(|| UrbanError::InvalidStateError)?
        .as_array()
        .ok_or_else(|| UrbanError::InvalidStateError)?
        .iter()
        .filter_map(|def| Definition::new(def))
        .collect())
}


// Errors

/// Errors for the library.
///
/// There are many different types of errors that can arrise when calling for definitions. Like a
/// reqwest error in case it can't access the online API, or a serde_json error when there's an
/// error in json parsing.
///
/// For this reason all the different possible errors are encapsulated under the `UrbanError` enum.
#[derive(thiserror::Error, Debug)]
pub enum UrbanError {
    #[error("reqwest error: {0:?}")]
    ReqwestError(reqwest::Error),
    #[error("serde_json error: {0:?}")]
    SerdeError(serde_json::Error),
    #[error("Invalid state")]
    InvalidStateError
}

impl convert::From<reqwest::Error> for UrbanError {
    fn from(error: reqwest::Error) -> Self {
        UrbanError::ReqwestError(error)
    }
}

impl convert::From<serde_json::Error> for UrbanError {
    fn from(error: serde_json::Error) -> Self {
        UrbanError::SerdeError(error)
    }
}
