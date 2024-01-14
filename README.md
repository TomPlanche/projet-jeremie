# Search for occurrences.
> This project is for a friend of mine.
> Its goal is to find the occurences of certain words or groups of words in a text file.

This is done in two steps:
- I first read [my friend's transcription](./src/assets/Transcription.docx) (in `.docx`) and convert it to a `.txt` file using a trivial [python script](./src/assets/main.py).
- I then use a [Rust program](./src/main.rs) to find the occurrences of the words I'm looking for.
