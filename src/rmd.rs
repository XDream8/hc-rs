// remove duplicates

// for file operations
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use crate::error::HcError;

pub fn remove_duplicates(file: &PathBuf) -> Result<(), HcError> {
    // open file
    let input: File = File::open(file).map_err(|e| HcError::FileWrite {
        filename: file.display().to_string(),
        source: e,
    })?;
    let reader: BufReader<File> = BufReader::new(input);

    // Collect lines into a vector
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // create output file
    let output: File = File::create(file)?;
    let mut writer: BufWriter<File> = BufWriter::new(output);

    let mut filtered: HashSet<String> = HashSet::new();

    for line in lines {
        if filtered.contains(&line) {
            continue;
        } else {
            writer
                .write_all(line.as_bytes())
                .map_err(|e| HcError::FileWrite {
                    filename: file.display().to_string(),
                    source: e,
                })?;
            writer
                .write_all(b"\n")
                .map_err(|e| HcError::FileWrite {
                    filename: file.display().to_string(),
                    source: e,
                })?;
            filtered.insert(line);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_remove_duplicates() {
        // Create test file
        let test_file = PathBuf::from("test_dedup.txt");
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"line1\nline2\nline1\nline3\nline2\nline3\n")
            .unwrap();
        drop(file);

        // Remove duplicates
        remove_duplicates(&test_file).unwrap();

        // Verify
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "line1\nline2\nline3\n");

        // Cleanup
        fs::remove_file(&test_file).ok();
    }
}
