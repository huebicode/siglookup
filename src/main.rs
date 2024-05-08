use std::env;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use colored::Colorize;
use rayon::prelude::*;
use walkdir::WalkDir;

mod check_pattern;
mod xmlsigparser;

mod search_sigs;
mod sigs_dynamic;
mod sigs_textorbin;
mod utils;

const COLUMN_FILE: usize = 30;
const COLUMN_SIZE: usize = 20;
const COLUMN_DESC: usize = 50;

fn main() {
    let sigs_file = env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get directory of current executable")
        .join("sigs.xml");

    if !sigs_file.exists() {
        println!("Signature file (sigs.xml) not found!");
        process::exit(0);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file- or dirpath>", &args[0]);
        process::exit(0);
    }

    let header = format!(
        "{:<COLUMN_FILE$} {:<COLUMN_SIZE$} {:<COLUMN_SIZE$} {:<COLUMN_SIZE$} {:<COLUMN_SIZE$} {:<COLUMN_DESC$}",
        "File", "Size", "Extension", "Category", "Platform", "Description",
        COLUMN_FILE = COLUMN_FILE, COLUMN_SIZE = COLUMN_SIZE, COLUMN_DESC = COLUMN_DESC
    );

    let dotted_line = "-".repeat(COLUMN_FILE + (COLUMN_SIZE * 4) + COLUMN_DESC);

    println!("{}", dotted_line.bright_black());
    println!("{}", header.bright_black());
    println!("{}", dotted_line.bright_black());

    let sigs = xmlsigparser::parse(sigs_file.to_str().unwrap());

    let file_counter = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&file_counter);

    let path = &args[1];
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            let file_extension = utils::get_file_extension(path);

            if path.is_file() {
                counter.fetch_add(1, Ordering::Relaxed);

                let result = search_sigs::check(&sigs, path.to_str().unwrap());
                let result: Vec<&str> = result.split('\t').collect();

                let file_name = result[0];
                let file_size = result[1];
                let extension = result[2].to_lowercase();
                let category = result[3];
                let platform = result[4];
                let description = result[5];

                let result_output = format!(
                    "{:<COLUMN_FILE$} {:<COLUMN_SIZE$} {:<COLUMN_SIZE$} {:<COLUMN_SIZE$} {:<COLUMN_SIZE$} {:<.COLUMN_DESC$}",
                    utils::truncate_with_ellipsis(file_name, COLUMN_FILE),
                    file_size,
                    extension,
                    category,
                    platform,
                    description,
                    COLUMN_FILE = COLUMN_FILE, COLUMN_SIZE = COLUMN_SIZE, COLUMN_DESC = COLUMN_DESC
                );

                if description.to_lowercase() == "unknown data" {
                    println!("{}", result_output.yellow());
                } else if description.to_lowercase() == "empty file" {
                    println!("{}", result_output.bright_cyan());
                } else if extension.contains(&file_extension) && !file_extension.is_empty(){
                    println!("{}", result_output.bright_green());
                } else if extension.contains("compressed") || extension.contains("zerobytes") {
                    println!("{}", result_output.bright_cyan());
                } else if extension.is_empty() || file_extension.is_empty(){
                        println!("{}", result_output);
                } else if !extension.contains(&file_extension) {
                    println!("{}", result_output.bright_red());
                } else {
                    println!("{}", result_output);
                }
            }
        });

    if file_counter.load(Ordering::Relaxed) > 1 {
        println!("{}", dotted_line.bright_black());
        let file_counter = format!("Files: {}\n", file_counter.load(Ordering::Relaxed));
        println!("{}", file_counter.cyan());
    } else {
        println!();
    }
}
