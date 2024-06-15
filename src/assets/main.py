"""
Script that extracts the text from the Word document and writes it in a text file.
This script will be run by the `main.rs` file so its output should be read by the Rust program.

Author:
    Tom Planche <tomplanche.fr|github.com/tomPlanche>
"""
from docx import Document
import os
import sys

# get the dir for setting the `os.chdir` to the right path
if len(sys.argv) > 1:
  path = sys.argv[1]

  if os.path.exists(path):
    os.chdir(path)

    document = Document('./Transcription.docx')

    all_text = [
        paragraphe.text
        for paragraphe in document.paragraphs
        if paragraphe.text != '' and paragraphe.text != '\n'
    ]

    # check if the file exists
    if not os.path.exists('../outputs'):
      os.mkdir('../outputs')

    with open('../outputs/results.txt', 'w') as f:
      f.write('\n'.join(all_text))
  else:
    print('The path does not exist')
else:
  print('You must provide a path as an argument')
