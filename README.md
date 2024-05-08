# Siglookup

Siglookup is a command-line tool for detecting file types based on their signatures (magic bytes).

![siglookup](./siglookup.png)

## Usage
The XML signature file (sigs.xml) must be located in the same directory as the application.

### Analyzing a file:
`siglookup <file>`

### Analyzing all files in a directory and its subdirectories:
`siglookup <dir>` or `siglookup .`

### Editing XML signatures:
Signatures can be created, expanded or changed in the XML file (sigs.xml) with the respective offset. Wildcards `_`, Ranges `-` and OR combinations `|` can be used for the magic bytes.

### Miscellaneous
If the file extension does not match the detected file extension, the output will appear in red. The application checks unrecognized files for compression/encryption (hight entropy) and zero bytes.

If you have file signatures, which the application is missing, find errors or have other requests, don’t hesitate to give me a note.

## Acknowledgments
The idea for this project arose when I discovered [Brian’s](https://github.com/brianary/magicnumber-lite) project while searching for some exotic file magic bytes. I liked the structure and recording of the signatures in his XML signature file, making them visible, searchable and expandable for everyone, which ultimately led to this project after a request to Brian. 

The signatures were mainly enriched by those of [Gerry Kessler](https://www.garykessler.net/library/file_sigs.html) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).