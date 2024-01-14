///
/// # main.rs
/// Little script to help Jeremie Arné for its project.
///
/// /// Tom Planche <github.com/tomPlanche>

// Imports  ==============================================================================  Imports use std::fs::File;
use std::{
    fs::File,
    io::{Read},
};
// Variables  =========================================================================== Variables
const ROOT_DIR: &str = "/Users/tom_planche/Desktop/Prog/Rust/projet-jeremie";

// Functions  =========================================================================== Functions
///
/// # search_occurences
/// Search the occurences of a string in a file.
/// The occurences should be case insensitive.
///
/// ## Arguments
/// * `text` - The text to search.
/// * `string` - The string to search.
///
/// ## Returns
/// The number of occurences.
fn search_occurences(
    text: &str,
    string: &str,
) -> usize {

}

// Main  ====================================================================================  Main
fn main() {
    // Change the current directory
    std::env::set_current_dir(ROOT_DIR).expect("Error while changing the current directory");

    // Read the file
    let mut file = File::open("./src/outputs/results.txt").expect("Error while opening the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error while reading the file");


}

/*
 * End of file src/main.rs
 */
