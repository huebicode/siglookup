# Siglookup

Siglookup is a command-line tool for detecting file types, based on their signatures (magic bytes).

![siglookup](https://github.com/huebicode/siglookup/assets/3885373/ed0ccf37-5c06-4dbc-bcf5-769b98c6f420)

## Usage
Important! The XML signature file (sigs.xml) must be located in the same directory as the application.
<br><br>

##### Analyzing a file
`siglookup <file>`
<br><br>

##### Analyzing all files in a directory and its subdirectories
`siglookup <dir>` or `siglookup .`
<br><br>

##### Editing XML signatures
Signatures can be created, expanded or changed in the XML file (sigs.xml) with the respective offset. Wildcards `_`, Ranges `-` and OR combinations `|` can be used.
<br><br>

##### Miscellaneous
If the file extension does not match the detected file extension, the output will appear in red. The application checks unrecognized files for compression/encryption (hight entropy) and zero bytes.

If you have file signatures, which the application is missing, find errors or have other requests, don’t hesitate to give me a note.
<br><br>

## Acknowledgments
The idea for this project arose when I discovered [Brian’s](https://github.com/brianary/magicnumber-lite) project while searching for some exotic magic bytes. I liked the structure and recording of the signatures in his XML signature file, making them visible, searchable and expandable for everyone, which ultimately led to this project after a request to Brian. 

The signatures were mainly enriched by those of [Gerry Kessler](https://www.garykessler.net/library/file_sigs.html) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
