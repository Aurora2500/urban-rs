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

/// A wrapper for the id of a definition entry.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Defid(u64);

impl Defid {
    /// defid constructor from u64
    pub fn new(id: u64) -> Defid {
        Defid(id)
    }

    /// Getter method to unwrap the u64 from a Defid.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// The struct to represent an Urban definition entry.
///
/// ## Example
///
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

impl Eq for Definition {}

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
    pub fn word(&self) -> &str {
        &self.word
    }

    /// The body of the definition
    pub fn definition(&self) -> &str {
        &self.definition
    }

    /// An example of the use of the word
    pub fn example(&self) -> &str {
        &self.example
    }

    /// The author of the Definition entry
    pub fn author(&self) -> &str {
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
    pub fn permalink(&self) -> &str {
        &self.permalink
    }

    /// A list of urls to sounds of the entry
    pub fn sound_urls(&self) -> &Vec<String> {
        &self.sound_urls
    }
}

// API Functions

/// Get a list of definitions trough a reqwest client by word.
///
/// ## Example
/// ```rust
/// use std::io;
///
/// use tokio::runtime::Runtime;
///
/// println!("What word do you want defined?");
/// let mut word = String::new();
/// io::stdin()
///     .read_line(&mut word)
///     .expect("Failed to read line");
///
/// // A reqwest client is needed to use the urban API
/// let client = reqwest::Client::new();
///
/// // The function is async. Thus it needs an executor to be ran from inside a non-async
/// // function.
/// if let Ok(result) = Runtime::new()
///     .expect("Failed to create tokio runtime")
///     .block_on(urban_rs::fetch_definition(&client, &word))
/// {
///
///     // the result is a vector of definitions. If it has no length then there were no words
///     // found
///     if result.is_empty() {
///         println!("No words were found");
///         return;
///     }
///
///     let first_result = &result[0];
///     println!("Definition for {}:\n{}", first_result.word(), first_result.definition());
///
/// } else {
///     println!("An error has occured while fetching the definition");
/// }
/// ```
/// This example asks the user for a word and returns the first definition from Urban Dictionary.
///
/// ## Errors
/// The error type of the result is UrbanError. Which is an enum of three types.
/// * ReqwestError
/// * SerdeError
/// * UnknownJsonError
///
/// ### ReqwestError
/// This error occurs when reqwest fails to fetch from the Urban API.
///
/// ### SerdeError
/// This error occurs when the json recieved is invalid.
///
/// ### UnknownJsonError
/// This error occurs when the json recieved is valid but does not have the expected structure.
///
/// ### Empty result
/// There is a fourth case. In which there are no entries in Urban Dictionary for the looked up
/// word. In which case the Vector returned will be empty.
///
/// ##
pub async fn fetch_definition(client: &reqwest::Client, word: &str) -> Result<Vec<Definition>, UrbanError> {
    let response: serde_json::Value = client.get(&format!("https://api.urbandictionary.com/v0/define?term={}", word))
        .send()
        .await?
        .json()
        .await?;

    response.get("list")
        .ok_or_else(|| UrbanError::UnknownJsonError)?
        .as_array()
        .ok_or_else(|| UrbanError::UnknownJsonError)?
        .iter()
        .map(|def| Definition::new(def).ok_or_else(|| UrbanError::UnknownJsonError))
        .collect()
}

/// Get a definition trough a reqwest client by Defid.
pub async fn fetch_by_defid(client: &reqwest::Client, defid: Defid) -> Result<Option<Definition>, UrbanError> {
    let response: serde_json::Value = client.get(&format!("https://api.urbandictionary.com/v0/define?defid={}", defid.0))
        .send()
        .await?
        .json()
        .await?;


    response.get("list")
        .ok_or_else(|| UrbanError::UnknownJsonError)?
        .as_array()
        .ok_or_else(|| UrbanError::UnknownJsonError)?
        .first()
        .map(|def| Definition::new(def).ok_or_else(|| UrbanError::UnknownJsonError))
        .transpose()
}
/// Fetch a list of random definitions trough a reqwest client.
///
/// ## Example
/// ```rust
/// use std::io;
///
/// use tokio::runtime::Runtime;
///
/// // A reqwest client is needed to use the urban API
/// let client = reqwest::Client::new();
///
/// // The function is async. Thus it needs an executor to be ran from inside a non-async
/// // function.
/// if let Ok(result) = Runtime::new()
///     .expect("Failed to create tokio runtime")
///     .block_on(urban_rs::fetch_random(&client))
/// {
///
///     // the result is a vector of definitions. If it has no length then there were no words
///     // found
///     if result.is_empty() {
///         println!("No words were found");
///         return;
///     }
///
///     let first_result = &result[0];
///     println!("Definition of the day: {}!\n{}", first_result.word(), first_result.definition());
///
/// } else {
///     println!("An error has occured while fetching the definition");
/// }
/// ```
pub async fn fetch_random(client: &reqwest::Client) -> Result<Vec<Definition>, UrbanError> {
    let response: serde_json::Value = client.get("https://api.urbandictionary.com/v0/random")
        .send()
        .await?
        .json()
        .await?;

    response.get("list")
        .ok_or_else(|| UrbanError::UnknownJsonError)?
        .as_array()
        .ok_or_else(|| UrbanError::UnknownJsonError)?
        .iter()
        .map(|def| Definition::new(def).ok_or_else(|| UrbanError::UnknownJsonError))
        .collect()
}


// Errors

/// Errors for the library.
///
/// There are many different types of errors that can arise when calling for definitions. Like a
/// reqwest error in case it can't access the online API, or a serde_json error when there's an
/// error in json parsing.
///
/// For this reason all the different possible errors are encapsulated under the `UrbanError` enum.
#[derive(thiserror::Error, Debug)]
pub enum UrbanError {
    /// Most likely produced when failing to fetch content from the internet.
    #[error("reqwest error: {0:?}")]
    ReqwestError(#[from] reqwest::Error),

    /// It would be produced when serde fails.
    #[error("serde_json error: {0:?}")]
    SerdeError(#[from] serde_json::Error),

    /// Error produced when the Json recieved from Urban's API has an unexpected structure.
    ///
    /// If a function returns this error. It means that it has correctly been able to fetch and
    /// recieve the Json from Urban's API. But it has not the expected structure containing the
    /// definitions.
    #[error("Valid json has unkown structure")]
    UnknownJsonError
}
