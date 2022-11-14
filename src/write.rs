// for Fetching urls
use crate::fetch::fetch;

// for file operations
use std::{fs::File, io::Write};

// colored output
use colored::*;

pub fn write(urls: String, filename: &str) {
    let mut body: String = String::new();

    let mut n: u8 = 1;
    let mut file = File::create(filename).expect("Error encountered while creating a file");

    for uri in urls.split(" ") {
        println!("{}) {}", format!("{}", n).cyan().bold(), uri.yellow());
        // Fetch url
        match fetch(&uri, &mut body) {
            Ok(f) => f,
            Err(e) => {
                println!("Couldn't fetch url!\n{:?}", e);
            }
        }
        // write to file
        match file.write_all(body.as_bytes()) {
            Ok(f) => f,
            Err(e) => {
                println!("Couldn't write fetched content to file!\n{:?}", e);
            }
        }
        n += 1;
    }
}
