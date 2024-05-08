use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zip::ZipArchive;

pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}…", &s[..max_len - 1])
    } else {
        s.to_string()
    }
}

pub fn get_file_extension(file_path: &Path) -> String {
    file_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_string()
        .to_lowercase()
}

pub fn is_utf8_continuation_byte(byte: u8) -> bool {
    (byte & 0b1100_0000) == 0b1000_0000
}

pub fn calculate_byte_entropy(data: &Vec<u8>) -> f64 {
    let mut frequencies = HashMap::new();

    for &byte in data {
        *frequencies.entry(byte).or_insert(0) += 1;
    }

    let total_bytes = data.len() as f64;

    frequencies.values().fold(0.0, |acc, &count| {
        let probability = count as f64 / total_bytes;
        acc - (probability * probability.log2())
    })
}

pub fn is_binary(data: &[u8]) -> bool {
    data.iter()
        .filter(|&&c| c < 32 && c != 9 && c != 10 && c != 13)
        .count()
        > data.len() / 50
}

pub fn zip_find_files(
    file_path: &Path,
    files_to_find: &[&str],
    optional_files: &[&str],
) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    let files_to_find_set: HashSet<String> = files_to_find.iter().map(|&s| s.into()).collect();
    let optional_files_set: HashSet<String> = optional_files.iter().map(|&s| s.into()).collect();

    let mut found_files = HashSet::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;

        let file_name = file.name().to_string();

        for pattern in files_to_find_set.iter() {
            if let Some(trimmed_pattern) = pattern.strip_prefix('*') {
                if file_name.ends_with(trimmed_pattern) {
                    found_files.insert(file_name.clone());
                    break;
                }
            } else if &file_name == pattern {
                found_files.insert(file_name.clone());
                break;
            }
        }

        if optional_files_set.contains(&file_name) {
            found_files.insert(file_name);
        }
    }

    Ok(found_files)
}

pub fn zip_get_filecontent(file_path: &Path, file_name: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.name() == file_name {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            return Ok(contents);
        }
    }

    Ok("".to_string())
}
