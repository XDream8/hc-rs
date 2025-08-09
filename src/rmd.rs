// remove duplicates

// for file operations
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Error, Write},
};

pub fn remove_duplicates(filename: &str) -> Result<(), Error> {
    // open file
    let input: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(input);

    // Collect lines into a vector
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // create output file
    let output: File = File::create(filename)?;
    let mut writer: BufWriter<File> = BufWriter::new(output);

    let mut filtered: HashSet<String> = HashSet::new();

    for line in lines {
        if filtered.contains(&line) {
            continue;
        } else {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
            filtered.insert(line);
        }
    }

    Ok(())
}
