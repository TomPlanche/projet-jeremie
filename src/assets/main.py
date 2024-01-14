"""
Script that extracts the text from the Word document and writes it in a text file.

Author:
    Tom Planche <tomplanche.fr|github.com/tomPlanche>
"""
from docx import Document
import os

os.chdir('/Users/tom_planche/Desktop/Prog/Rust/projet-jeremie/src/assets')

document = Document('./Transcription.docx')

all_text = [
    paragraphe.text
    for paragraphe in document.paragraphs
    if paragraphe.text != '' and paragraphe.text != '\n'
]

with open('../outputs/results.txt', 'w') as f:
    f.write('\n'.join(all_text))
