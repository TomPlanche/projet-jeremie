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

    /// Max distance
    /// Optional 'max-distance' argument, default value is 2.
    #[arg(short, long, default_value = "2")]
    max_distance: usize,

    /// Print the vector of occurences
    /// Optional 'print-occurences' argument.
    #[arg(short, long)]
    print_occurences: bool,
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
) -> usize {
    let words_iter = line.split_whitespace();
    let window_size = string.split_whitespace().count();

    let mut matches: Vec<(usize, String)> = Vec::new();

    for window in words_iter.collect::<Vec<&str>>().windows(window_size) {
        let window = window.join(" ");
        let distance = edit_distance(&window, string);

        if distance <= max_distance {
            matches.push((distance, window));
        }
    }

    matches.len()
}

// Main  ====================================================================================  Main
fn main() {
    // Change the current directory
    std::env::set_current_dir(ROOT_DIR).expect("Error while changing the current directory");

    // Cli
    let cli = Cli::parse();
    let file = cli.file;
    let debug = cli.debug;
    let max_distance = cli.max_distance;
    let print_occurences = cli.print_occurences;

    // Read the file
    let mut file = File::open(file).expect("Error while opening the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error while reading the file");

    let mut occurences: HashMap<&str, (u8, (usize, Vec<String>))> = HashMap::new();

    for line in content.lines() {
        if debug {
            println!("-------------------- line: {}", line);
        }
        for guy in GUYS_TO_SEARCH.iter() {
            if debug {
                println!("----- guy: {}", guy);
            }
            let matches = find_approx_match(&line, guy, max_distance);

            if matches > 0 {
                if debug {
                    println!("{} is in the line", guy);
                }

                let occurence = occurences.entry(guy).or_insert((0, (0, Vec::new())));
                occurence.0 += matches as u8;
                occurence.1.0 = matches;
                occurence.1.1.push(line.to_string());
            }
        }

        for town in TOWNS_TO_SEARCH.iter() {
            if debug {
                println!("----- town: {}", town);
            }

            let matches = find_approx_match(&line, town, max_distance);

            if matches > 0 {
                if debug {
                    println!("{} is in the line", town);
                }

                let occurence = occurences.entry(town).or_insert((0, (0, Vec::new())));
                occurence.0 += matches as u8;
                occurence.1.0 = matches;
                occurence.1.1.push(line.to_string());
            }
        }
    }

    println!("Occurences:");
    if print_occurences {
        println!("{:#?}", occurences);
    } else {
        for (key, (cpt, _)) in occurences.iter() {
            println!("{}: {}", key, cpt);
        }
    }
}

#[cfg(test)]
mod tests {}
/*
 * End of file src/main.rs
 */
