use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use num_format::SystemLocale;
use num_format::ToFormattedString;

use crate::sigs_dynamic;
use crate::xmlsigparser;
use crate::{check_pattern, sigs_textorbin};

pub fn check(sigs: &xmlsigparser::Magic, file_path: &str) -> String {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let path = Path::new(&file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let file_size = reader.get_ref().metadata().unwrap().len();
    if file_size == 0 {
        return format!("{}\t{}\tempty\t-\t-\tEmpty File", file_name, file_size);
    }

    let formatted_size = file_size.to_formatted_string(&SystemLocale::default().unwrap());

    // check for magic bytes against dynamic sigs
    let result = sigs_dynamic::check(&mut reader, file_size, path);
    if result != "NOHIT" {
        return format!("{}\t{}\t{}", file_name, formatted_size, result);
    }

    // check for magic bytes against the xml sig file
    for offset in &sigs.offsets {
        if offset.at.starts_with('-') {
            match offset.at.parse::<i64>() {
                Ok(offset_value) => {
                    if offset_value.unsigned_abs() > file_size {
                        continue;
                    } else {
                        reader.seek(SeekFrom::End(offset_value)).unwrap();
                    }
                }
                Err(_) => continue,
            };
        } else {
            match offset.at.parse::<u64>() {
                Ok(offset_value) => {
                    if offset_value > file_size {
                        continue;
                    } else {
                        reader.seek(SeekFrom::Start(offset_value)).unwrap();
                    }
                }
                Err(_) => continue,
            };
        }

        const BUFFER_SIZE: usize = 64;
        let mut buffer = [0; BUFFER_SIZE];

        let _bytes_read = reader.read(&mut buffer).unwrap(); // set to a _var to satisfy clippy::unused_io_amount
        let hex_file_bytes: Vec<String> = buffer.iter().map(|b| format!("{:02X}", b)).collect();

        for first in &offset.first {
            if first.byte != hex_file_bytes[0] {
                continue;
            }

            for _match in &first.matches {
                let pattern_bytes: Vec<&str> = _match.bytes.split_whitespace().collect();
                if file_size < pattern_bytes.len() as u64 {
                    continue;
                }

                if check_pattern::is_pattern_in_hex(&hex_file_bytes, &_match.bytes) {
                    return format!(
                        "{}\t{}\t{}\t{}\t{}\t{}",
                        file_name, formatted_size, _match.ext, _match.cat, _match.os, _match.info
                    );
                }
            }
        }
    }

    // check for text and binary
    let result = sigs_textorbin::check(&mut reader, file_size);
    if result != "NOHIT" {
        return format!("{}\t{}\t{}", file_name, formatted_size, result);
    }

    // if no match found, return unknown data
    format!(
        "{}\t{}\tunknown\t-\t-\tUnknown Data",
        file_name, formatted_size
    )
}
