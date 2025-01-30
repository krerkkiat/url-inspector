use std::fs;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// pptx file to check
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if cli.file.extension().expect("file is not a .pptx file.") != "pptx" {
        println!("file is not a .pptx file.");
    }

    let archive_file_handle = fs::File::open(cli.file).expect("failed to open file.");
    let mut archive =
        zip::ZipArchive::new(archive_file_handle).expect("failed to open .pptx as archive file.");

    let filename_pattern =
        Regex::new(r"(notes)?[s|S]lide\d+\.xml").expect("failed to create regex.");
    let url_pattern = Regex::new(
        r"https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)",
    ).expect("failed to create regex.");

    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(f) => f,
            Err(_) => continue,
        };
        let file_path = match file.enclosed_name() {
            Some(p) => p,
            None => continue,
        };

        // Skip any directory.
        if file.is_dir() {
            continue;
        }
        let file_name = match file_path.file_stem() {
            Some(p) => match p.to_str() {
                Some(p) => p,
                None => continue,
            },
            None => continue,
        };

        // Skip any file that is not one of the following
        // - slide<N>.xml
        // - notesSlide<N>.xml
        if !filename_pattern.is_match(file_name) {
            continue;
        }

        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Err(_) => continue,
            _ => (),
        }

        let urls: Vec<&str> = url_pattern.find_iter(&buffer).map(|m| m.as_str()).collect();
        for i in 0..urls.len() {
            if urls[i].starts_with("http://schemas.openxmlformats.org") {
                continue;
            }
            let result: i32 = match check_url(urls[i]) {
                Some(code) => code.into(),
                None => -1,
            };
            if result != 200 {
                println!("{} '{}' in {}", result, urls[i], file_name);
            }
        }
    }
}

fn check_url(url: &str) -> Option<u16> {
    match reqwest::blocking::get(url) {
        Ok(res) => Some(res.status().as_u16()),
        Err(_) => None,
    }
}
