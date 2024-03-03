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
// Imports  ==============================================================================  Imports
use clap::Parser;
use edit_distance::edit_distance;
use std::{
    collections::HashMap,
    env::{current_dir, set_current_dir},
    fs::File,
    io::Read,
    path::PathBuf,
    process::Command,
};

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

    /// Output the vector of occurences in a json file
    /// Optional 'output-occurences' argument.
    #[arg(short, long)]
    output_occurences: bool,

    /// Preruns the transcription script
    /// Optional 'run-transcription' argument.
    #[arg(short, long)]
    run_transcription: bool,
}

// Type(s)
type Match = Vec<String>;
type Occurences<'a> = HashMap<&'a str, Match>;

pub type Strings2Search = HashMap<String, u16>;

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
/// * `line` - `&str` - The line to search.
/// * `string` - `&str` - The string to search.
/// * `max_distance` - `u16` - The maximum distance between the string and the line.
///
/// ## Returns
/// * `(u16, Vec<String>` - The number of occurences and the occurences.
fn find_approx_match(line: &str, string: &str, max_distance: &u16) -> (u16, Vec<String>) {
    let words_iter = line.split_whitespace();
    let window_size = string.split_whitespace().count();

    words_iter.collect::<Vec<&str>>().windows(window_size).fold(
        (0u16, Vec::new()),
        |mut matches, window| {
            let window = window.join(" ");
            let distance = edit_distance(&window, string) as u16;

            if distance <= *max_distance {
                matches.0 += 1;
                matches.1.push(window);
            }

            matches
        },
    )
}

///
/// # load_from_json
/// Load a json file and return a HashMap.
/// The json file must look like this:
/// ```json
/// {
///   "test_2_search": 2 // where the number is the allowed 'errors' possible for that word.
/// }
/// ```
///
/// ## Arguments
/// * `file_path` - `&PathBuf` - The path to the json file.
///
/// ## Returns
/// * `Strings2Search` - The loaded json file typed as a `Strings2Search`
fn load_strings_to_search(file_path: &PathBuf) -> Strings2Search {
    // Open the file
    let mut file = File::open(file_path).expect("The file could not be opened");

    // Read the contents of the file into a string
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .expect("The file could not be read");

    serde_json::from_str(&json_string).expect("The json file is not valid")
}

///
/// # export_to_json
/// Export the final result to a json file.
///
/// ## Arguments
/// * `occurences` - `Occurences` - The occurences to export.
/// * `file_path` - `&PathBuf` - The path to the file to create.
fn export_to_json(occurences: Occurences, file_path: &PathBuf) {
    // Open the file
    let file = File::create(file_path).expect("The file could not be created");

    // Write the occurences to the file
    serde_json::to_writer_pretty(&file, &occurences)
        .expect("The occurences could not be written to the file");
}

///
/// # run_python_script
/// Run the transcription script, written in python.
/// The python script takes the path to the assets folder as an argument.
fn run_python_script() {
    let assets_path = PathBuf::from(ROOT_DIR).join("src/assets");
    let python_script_path = assets_path.join("main.py");

    let _ = Command::new("python3")
        .arg(python_script_path)
        .arg(assets_path)
        .output()
        .expect("Error while running the python script");
}

// Main  ====================================================================================  Main
fn main() {
    let caller = current_dir().unwrap();

    // Change the current directory
    set_current_dir(caller).expect("Error while changing the current directory");

    // Cli
    let cli = Cli::parse();
    let file_2_read = cli.file_2_read;
    let debug = cli.debug;
    let print_occurences = cli.print_occurences;
    let strings_file = cli.strings_file;
    let output_occurences = cli.output_occurences;
    let run_transcription = cli.run_transcription;

    if run_transcription {
        run_python_script();
    }

    // Read the transcript
    let mut file = File::open(file_2_read).expect("Error while opening the file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Error while reading the file");

    // Load the strings to search
    let strings: Strings2Search = load_strings_to_search(&PathBuf::from(strings_file));

    let mut occurences: Occurences = HashMap::new();

    for line in content.lines() {
        if debug {
            println!("-------------------- line: {}", line);
        }

        for string in &strings {
            let (cpt, matches) = find_approx_match(line, string.0, string.1);

            if cpt > 0 {
                if debug {
                    println!("{}: {}", string.0, cpt);
                }

                let total_cpt = occurences.entry(string.0).or_insert(Vec::new());
                total_cpt.extend(matches);
            }
        }
    }

    if print_occurences {
        println!("Occurences:");
        println!("{:#?}", occurences);
    } else {
        for (string, matches) in &occurences {
            println!("{}: {}", string, matches.len());
        }
    }

    if output_occurences {
        export_to_json(occurences, &PathBuf::from("./src/outputs/occurences.json"));
    }
}

#[cfg(test)]
mod tests {
    use super::Strings2Search;
    use super::{find_approx_match, load_strings_to_search};
    use std::path::PathBuf;

    #[test]
    fn test_read_json() {
        let file_path = PathBuf::from("./src/assets/toFind.json");
        let text: Strings2Search = load_strings_to_search(&file_path);

        assert_eq!(text.get("Jehan de Luxembourg"), Some(&4u16));
    }

    #[test]
    fn test_find_approx_match() {
        let line = "Le vallet Jehan de Luxembourcq pris son arme.";
        let string = "Jehan de Luxembourc";
        let max_distance = 3;

        // The edit distance between "Jehan de Luxembourg" and "Jehan de Luxembourc" is 1,
        // so the function will return true because 1 <= 3.
        assert_eq!(
            find_approx_match(line, string, &max_distance),
            (1, vec!["Jehan de Luxembourcq".to_string()])
        );
    }
}

/*
 * End of file src/main.rs
 */
