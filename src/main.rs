///
/// # main.rs
/// Little script to help Jeremie Arné for its project.
///
/// ## Description
/// This script will read the file `./src/outputs/results.txt` and search the occurences of the
/// needed strings.
///
/// ## Author
/// Tom Planche <tomplanche.fr|github.com/tomPlanche>

// Imports  ==============================================================================  Imports use std::fs::File;
use std::{
    fs::File,
    io::{Read},
};
use std::collections::HashMap;
use edit_distance::edit_distance;
use clap::{Parser};

// Variables  =========================================================================== Variables
const ROOT_DIR: &str = "/Users/tom_planche/Desktop/Prog/Rust/projet-jeremie";

const GUYS_TO_SEARCH: [&str; 4] = [
    "Jehan de Luxembourg",
    "Duc de Bourgogne",
    "le bastard de Thyan",
    "Saint Pol",
];

const TOWNS_TO_SEARCH: [&str; 3] = [
    "Tournay",
    "Cambrai",
    "Haynaut",
];

// Cli parser
#[derive(Parser)]
#[command(about = "Small script that will seek the occurences of the needed strings in a file.")]
#[command(author = "Tom Planche <tomplanche.fr|github.com/tomPlanche>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
#[command(name = "find-occurences")]
struct Cli {
    /// File to read
    /// Optional 'file' argument, default value is './src/outputs/results.txt'.
    #[arg(short, long, default_value = "./src/outputs/results.txt")]
    file: String,

    /// Debug mode
    /// Optional 'debug' argument.
    #[arg(short, long)]
    debug: bool,

}

// Functions  =========================================================================== Functions
///
/// # find_approx_match
/// Find the occurences of a string in a line.
/// The string can be approximated.
///
/// ## Example
/// ```
/// let line = "prins par messire Jehan de Luxembourg et autres";
/// let string = "Jehan de Luxembourc";
/// let max_distance = 3;
///
/// // The edit distance between "Jehan de Luxembourg" and "Jehan de Luxembourc" is 1,
/// // so the function will return true because 1 <= 3.
/// assert_eq!(find_approx_match(line, string, max_distance), true);
/// ```
///
/// ## Arguments
/// * `line` - The line to search in.
/// * `string` - The string to search.
/// * `max_distance` - The maximum distance between the string and the line.
///
/// ## Returns
/// The number of occurences.
fn find_approx_match(
    line: &str,
    string: &str,
    max_distance: usize,
) -> bool {
    let mut words_iter = line.split_whitespace();
    let window_size = string.split_whitespace().count();

    let mut smallest_match: (usize, String) = (999, words_iter.by_ref().take(window_size).collect::<Vec<&str>>().join(" "));

    for window in words_iter.collect::<Vec<&str>>().windows(window_size) {
        let window = window.join(" ");
        let distance = edit_distance(&window, string);

        if distance < smallest_match.0 {
            smallest_match = (distance, window);
        }
    }

    smallest_match.0 <= max_distance
}

// Main  ====================================================================================  Main
fn main() {
    // Change the current directory
    std::env::set_current_dir(ROOT_DIR).expect("Error while changing the current directory");

    let cli = Cli::parse();

    // Read the file
    let mut file = File::open(cli.file).expect("Error while opening the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error while reading the file");

    let mut occurences: HashMap<&str, usize> = HashMap::new();

    let debug: bool = cli.debug;

    for line in content.lines() {
        if debug {
            println!("-------------------- line: {}", line);
        }
        for guy in GUYS_TO_SEARCH.iter() {
            if debug {
                println!("----- guy: {}", guy);
            }
            if find_approx_match(&line, guy, 2) {
                if debug {
                    println!("{} is in the line", guy);
                }

                let occurence = occurences.entry(guy).or_insert(0);
                *occurence += 1;
            }
        }

        for town in TOWNS_TO_SEARCH.iter() {
            if debug {
                println!("----- town: {}", town);
            }

            if find_approx_match(&line, town, 2) {
                if debug {
                    println!("{} is in the line", town);
                }

                let occurence = occurences.entry(town).or_insert(0);
                *occurence += 1;
            }
        }
    }

    println!("occurences: {:?}", occurences);
}

#[cfg(test)]
mod tests {}
/*
 * End of file src/main.rs
 */
