# Search for occurrences.
> This project is for Jérémie Arné's research thesis of Master 1. <br>
> Its goal is to find the occurences of certain words or groups of words in a text file.

## Steps
- I first read [Jérémie's transcription](./src/assets/Transcription.docx) (in `.docx`) and convert it to a `.txt` file using a trivial [python script](./src/assets/main.py).
- I then use a [Rust program](./src/main.rs) to find the occurrences of the words I'm looking for.

## Usage
```text
projet-jeremie [OPTIONS]

Options:
  -f, --file-2-read <FILE_2_READ>    File to read Optional 'file' argument, default value is './src/outputs/results.txt' [default: ./src/outputs/results.txt]
  -s, --strings-file <STRINGS_FILE>  File that contains the strings to search. Optional 'strings-file' argument, default value is './src/outputs/strings.txt' [default: ./src/assets/toFind.json]
  -d, --debug                        Debug mode Optional 'debug' argument
  -p, --print-occurences             Print the vector of occurences Optional 'print-occurences' argument
  -h, --help                         Print help
```
### JSON file
The JSON file for the strings to search must be an array of objects of the following format:
```json
{
  "string": "string to search",
  "max_distance": 4 // arbitrary value
}
```

## Algorithm
The word `algorithm` is a bit of a stretch here.
All I'm doing is reading the file line by line and for each line, I in the line all the windows of words I'm looking for.

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
