# urban-rs: API for Urban Dictionary

An async API to interact with Urban Dictionary to get definitions from it.

The API Uses reqwest for fetching definitions off of the internet

There are three ways to get a definition:

* by word
* by id
* randomly

## Example
```rust
use std::io;

use tokio::runtime::Runtime;

println!("What word do you want defined?");
let mut word = String::new();
io::stdin()
    .read_line(&mut word)
    .expect("Failed to read line");

// A reqwest client is needed to use the urban API
let client = reqwest::Client::new();

// The function is async. Thus it needs an executor to be ran from inside a non-async
// function.
if let Ok(result) = Runtime::new()
    .expect("Failed to create tokio runtime")
    .block_on(urban_rs::fetch_definition(&client, &word))
{

    // the result is a vector of definitions. If it has no length then there were no words
    // found
    if result.is_empty() {
        println!("No words were found");
        return;
    }

    let first_result = &result[0];
    println!("Definition for {}:\n{}", first_result.word(), first_result.definition());

} else {
    println!("An error has occured while fetching the definition");
}
```
This example asks a user for a word and prints out its definition

## Guide
To start using urban-rs. Add it to your cargo.toml

```toml
urban-rs = "0.1.0"
```
Urban-rs uses `reqwest` to fetch definitions trough the internet asynchronously.

This means that you will need to use a `reqwest::Client` to give to the functions.
The reasons for the user to provide a client is so that it can be reused across multiple function calls.
Or even across different APIs that all need a reqwest client.

Aditionally, since the functions are asynchronous, they won't directly return a result.
But instead will return a Future that needs to be executed.
Using `futures`'s executors won't work. As reqwest requires tokios runtime to be executed.
Thus the futures returned from the functions need to be called trough `tokio`'s `Runtime` and its executors.

```rust
use tokio::runtime::Runtime;

// A reqwest client is needed so that the Urban API can make web API calls
let client = reqwest::Client::new();

// As stated before. The API uses async functions which return futures. These need to be executed trough
// tokio's Runtime.
if let Ok(result) = Runtime::new()
    .expect("Failed to create tokio runtime")
    .block_on(urban_rs::fetch_random(&client))
{

    // the result is a vector of definitions. If it has no length then there were no words
    // found
    if result.is_empty() {
        println!("No words were found");
        return;
    }

    let first_result = &result[0];
    println!("Definition of the day: {}!\n{}", first_result.word(), first_result.definition());

} else {
    println!("An error has occured while fetching the definition");
}
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
