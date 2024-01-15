# Search for occurrences.
> This project is for Jérémie Arné. <br>
> Its goal is to find the occurences of certain words or groups of words in a text file.

## Steps
- I first read [Jérémie's transcription](./src/assets/Transcription.docx) (in `.docx`) and convert it to a `.txt` file using a trivial [python script](./src/assets/main.py).
- I then use a [Rust program](./src/main.rs) to find the occurrences of the words I'm looking for.

## Algorithm
The word `algorithm` is a bit of a stretch here.
All I'm doing is reading the file line by line and for each line, I check within the window of the needed words if they are present.

### Example
Sometimes, words are written with different spellings.
For example, `Jehan de Luxembourg` can be found as `Jehan de Luxembourcq` or `Jehan de Luxembouc`
