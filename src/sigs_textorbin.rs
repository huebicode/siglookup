use serde_json::Value as JsonValue;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::utils;

pub fn check(reader: &mut BufReader<File>, file_size: u64) -> &str {
    // read-in file bytes
    reader.seek(SeekFrom::Start(0)).unwrap();

    let mut buffer = [0; 1];

    // 10MB max read-in size
    let byte_read_limit = std::cmp::min(file_size, 10485760);

    let mut collected_bytes = Vec::with_capacity(byte_read_limit as usize);
    let mut read_counter = 0;

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if read_counter > byte_read_limit && !utils::is_utf8_continuation_byte(buffer[0]) {
            break;
        }

        collected_bytes.push(buffer[0]);
        read_counter += 1;
    }

    // check file bytes type
    if utils::is_binary(&collected_bytes) {
        if collected_bytes.iter().all(|&b| b == 0) {
            "zerobytes\t-\t-\tFile filled with Zero-Bytes"
        } else if utils::calculate_byte_entropy(&collected_bytes) > 7.5 {
            "compressed/encrypted\t-\t-\tFile with High-Entropy"
        } else {
            "NOHIT"
        }
    } else if serde_json::from_slice::<JsonValue>(&collected_bytes).is_ok() {
        "json\tText\tMisc\tJSON File"
    } else if collected_bytes.is_ascii() {
        "txt/text\tText\tMisc\tASCII Text File"
    } else if std::str::from_utf8(&collected_bytes).is_ok() {
        "txt/text\tText\tMisc\tUnicode Text (UTF-8)"
    } else {
        "txt/text\tText\tMisc\tExtended ASCII Text File"
    }
}
