// remove duplicates

// for file operations
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Error, Write},
};

// fast data type
use bloomy_rs::BloomFilter;

pub fn remove_duplicates(filename: &str) -> Result<(), Error> {
    // open file
    let input: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(input);

    // Collect lines into a vector
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Estimate number of distinct elements for the Bloom filter
    let estimated_distinct_elements: usize = lines.len() * 2;
    const FILTER_SIZE: usize = 1024 * 1024 * 8;
    // // Adjust bloom filter as needed
    let mut bloom_filter: BloomFilter<FILTER_SIZE> = BloomFilter::new(estimated_distinct_elements);

    // create output file
    let output: File = File::create(filename)?;
    let mut writer: BufWriter<File> = BufWriter::new(output);

    for line in lines {
        if bloom_filter.contains(&line) {
            continue;
        } else {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
            bloom_filter.insert(&line);
        }
    }

    Ok(())
}
