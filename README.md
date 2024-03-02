# Search for occurrences.

> This project is for _*Jérémie Arné*_'s research thesis of Master 1. <br>
> Its goal is to find the occurences of certain words or groups of words in a text file.

## Requirements

You'll need these two languages installed and ready.

- [Rust](https://www.rust-lang.org/tools/install)
- [Python](https://www.python.org/downloads/)

## Steps

- I first read [Jérémie's transcription](./src/assets/Transcription.docx) (`.docx`) and convert it to a `.txt` file using a trivial [python script](./src/assets/main.py).
- I then use a [Rust program](./src/main.rs) to find the occurrences of the words I'm looking for.

## Usage

### Before running the script.

In order to make it easy to use for people that are not familiar with code and a terminal (_Jérémie_), I automated amost all the process.

In order to use it, you'll just need to:

- Make sure the transcription file is in the `./src/assets/` folder.
- (Create or) fill the `src/assets/toFind.json` file.
  It should have the following structure:

  ```json
  {
    "the_word_to_look_for": 4, // This number is the maximum errors possible in that word.
    "another_word or expression": 5
  }
  ```

### Compiling

You'll need to do this only _*ONCE*_.

```bash
cargo build --release
```

### Running the script

> The script takes sevral arguments that can be found using this code:
>
> ```
> ./target/release/projet-jeremie -h
> ```

[After making sure all configuration files are OK](#Before running the script.), the easiest way to get things working is via this command:

```
./target/release/projet-jeremie -ro
```

This command will:

- `-r` **R**un the python script to convert the `.docx` transcription file into a `.txt` one.
- `o` Will **o**utput the results in the `src/outputs/occurences.json` file.

## JSON file

The JSON file for the strings to search must an object of `"string": number` like so:

```json
{
  "Jehan de Luxembourg": 4,
  "Duc de Bourgogne": 3
}
```

The numbers are here to precise the maximum number of errors for a given string.

## Algorithm

The word `algorithm` is a bit of a stretch here.
All I'm doing is reading the file line by line and for each line, I'm looking for the occurences of the words I'm looking for uing windows of the size of the word(s) I'm looking for.

### Example

Sometimes, words are written with different spellings.
For example, `Jehan de Luxembourg` can be found as `Jehan de Luxembourcq` or `Jehan de Luxembouc`.

In the line `Le vallet Jehan de Luxembourcq pris son arme.`, given the `Jehan de Luxembourg` search, the looking window will be of size 3. And the program will browse the line like this:

- Le vallet Jehan | distance: 16
- vallet Jehan de | distance: 16
- Jehan de Luxembourcq | distance: 1
- de Luxembourcq pris | distance: 12
- Luxembourcq pris son | distance: 19
- pris son arme. | distance: 17

If the distance is less than the maximum distance allowed, the program will take it into account.
If multiple occurences are found, the program will also take it into account.
