// remove duplicates

// for file operations
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn remove_duplicates(filename: &str) {
    // open file
    let file = File::open(filename).expect("Couldn't open file");
    let reader = BufReader::new(file);
    // read all uniq lines
    let lines: BTreeSet<_> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line"))
        .collect();

    let mut file = File::create(filename).expect("Error encountered while creating a file");
    for line in lines {
        // write uniq line
        file.write_all(line.as_bytes())
            .expect("Couldn't write to file");

        // write newline(\n)
        file.write_all(b"\n").expect("Couldn't write to file");
    }
}
