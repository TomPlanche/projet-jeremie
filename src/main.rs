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
    collections::HashMap,
    path::PathBuf,
};
use edit_distance::edit_distance;
use clap::{Parser};
use serde::{Deserialize, Serialize};

// Variables  =========================================================================== Variables
const ROOT_DIR: &str = "/Users/tom_planche/Desktop/Prog/Rust/projet-jeremie";


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
    file_2_read: String,

    /// File that contains the strings to search.
    /// Optional 'strings-file' argument, default value is './src/outputs/strings.txt'.
    #[arg(short, long, default_value = "./src/assets/toFind.json")]
    strings_file: String,

    /// Debug mode
    /// Optional 'debug' argument.
    #[arg(short, long)]
    debug: bool,

    /// Print the vector of occurences
    /// Optional 'print-occurences' argument.
    #[arg(short, long)]
    print_occurences: bool,
}

// Types
#[derive(Serialize, Deserialize, Debug)]
struct SearchString {
    string: String,
    max_distance: usize,
}

type Occurences<'a> = HashMap<
    &'a str,
    (
        u8,
        Vec<
            (
                usize,
                String
            )
        >
    )
>;
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

///
/// # load_from_json
/// Load a json file and return a HashMap.
/// The json file must look like this:
/// [
///   {
//      "string": "Jehan de Luxembourg",
//      "max_distance": 4
//    },
/// ]
///
/// ## Arguments
/// * `file_path` - The path to the json file.
///
/// ## Returns
/// A Vec of SearchString.
fn load_from_json(
    file_path: &PathBuf,
) -> Vec<SearchString> {
    // Open the file
    let mut file = File::open(file_path).expect("The file could not be opened");

    // Read the contents of the file into a string
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).expect("The file could not be read");

    // the json file is an array of objects
    let json: Vec<SearchString> = serde_json::from_str(&json_string).expect("The json file is not valid");

    json
}
// Main  ====================================================================================  Main
fn main() {
    // Change the current directory
    std::env::set_current_dir(ROOT_DIR).expect("Error while changing the current directory");

    // Cli
    let cli = Cli::parse();
    let file_2_read = cli.file_2_read;
    let debug = cli.debug;
    let print_occurences = cli.print_occurences;
    let strings_file = cli.strings_file;

    // Read the transcript
    let mut file = File::open(file_2_read).expect("Error while opening the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error while reading the file");

    // Load the strings to search
    let strings = load_from_json(&PathBuf::from(strings_file));

    // Occurences:
    // HashMap<&str, (u8, (usize, Vec<String>))>
    // u8: number of total occurences
    // usize: number of occurences in a line
    // Vec<String>: the lines where the string was found
    let mut occurences: Occurences = HashMap::new();

    for line in content.lines() {
        if debug {
            println!("-------------------- line: {}", line);
        }

        for string in &strings {
            let cpt = find_approx_match(line, &string.string, string.max_distance);

            if cpt > 0 {
                if debug {
                    println!("{}: {}", string.string, cpt);
                }

                let (total_cpt, lines) = occurences.entry(&string.string).or_insert((0, Vec::new()));
                *total_cpt += cpt as u8;
                lines.push((cpt, line.to_string()));

                if debug {
                    println!("total_cpt: {} | lines: {:?}", total_cpt, lines);

                }
            }
        }
    }

    println!("Occurences:");
    if print_occurences {
        println!("{:#?}", occurences);
    } else {
        for (string, (total_cpt, _)) in occurences {
            println!("{}: {}", string, total_cpt);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    #[test]
    fn test_read_json() {
        let file_path = PathBuf::from("./src/assets/toFind.json");
        let text = super::load_from_json(&file_path);

        assert_eq!(text[0].string, "Jehan de Luxembourg");
    }
}
/*
 * End of file src/main.rs
 */
