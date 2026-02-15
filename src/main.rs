mod cli;
mod error;
mod fetch;
mod rmd;

use clap::Parser;
use error::HcError;

use cli::CliArgs;
use fetch::fetch;

use std::collections::HashSet;

// for file operations
use std::{fs::File, io::Write};

use std::sync::LazyLock;

// http client
use ureq::Agent;

use crate::cli::Commands;
use crate::rmd::remove_duplicates;

// reusable lazy initialized HTTP CLIENT
pub static HTTP_CLIENT: LazyLock<Agent> = LazyLock::new(Agent::new_with_defaults);

fn main() -> Result<(), HcError> {
    let cli: CliArgs = CliArgs::parse();

    match &cli.command {
        Some(Commands::Rmd { files }) => Ok(files.iter().try_for_each(remove_duplicates)?),
        _ => {
            // Determine URLs
            let urls: Vec<&str> = if !cli.urls.is_empty() {
                cli.urls.iter().map(|s| s.as_str()).collect()
            } else {
                vec![
                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
                    "https://raw.githubusercontent.com/notracking/hosts-blocklists/master/hostnames.txt",
                    "https://raw.githubusercontent.com/jerryn70/GoodbyeAds/master/Hosts/GoodbyeAds.txt",
                ]
            };

            create_hosts_file(
                cli.remove_duplicate_lines,
                cli.output_file,
                cli.ignore_fetching_errors,
                &urls,
            )
        }
    }
}

fn create_hosts_file(
    remove_duplicate_lines: bool,
    output_file: String,
    ignore_fetching_errors: bool,
    hosts: &[&str],
) -> Result<(), HcError> {
    // give info
    println!(
        "Filename: {}\nRemove Duplicates: {}",
        output_file, remove_duplicate_lines
    );

    // Create output file
    File::create(&output_file).map_err(|e| HcError::FileCreation {
        filename: output_file.clone(),
        source: e,
    })?;

    println!("Starting downloads(threaded)");

    // Fetch in parallel
    let fetched_content = fetch_all_hosts(hosts, ignore_fetching_errors)?;

    // open file as read-write
    let mut file = File::options()
        .append(true)
        .open(&output_file)
        .map_err(|e| HcError::FileWrite {
            filename: output_file.to_string(),
            source: e,
        })?;

    // remove duplicates if -rmd flag is used
    if remove_duplicate_lines && hosts.len() == 1 {
        println!("We only fetched 1 hosts file, no need to remove duplicates");
    } else if remove_duplicate_lines {
        println!("Removing duplicate lines and writing to file");

        let mut filtered: HashSet<String> = HashSet::new();

        for content in fetched_content {
            let content_by_lines: Vec<String> = content.lines().map(String::from).collect();

            for line in content_by_lines {
                if filtered.contains(&line) {
                    continue;
                } else {
                    file.write_all(line.as_bytes())
                        .map_err(|e| HcError::FileWrite {
                            filename: output_file.to_string(),
                            source: e,
                        })?;
                    file.write_all(b"\n").map_err(|e| HcError::FileWrite {
                        filename: output_file.to_string(),
                        source: e,
                    })?;

                    filtered.insert(line);
                }
            }
        }
    } else {
        println!("Writing to file!");

        for data in fetched_content {
            file.write_all(data.as_bytes())
                .map_err(|e| HcError::FileWrite {
                    filename: output_file.to_string(),
                    source: e,
                })?;
        }
    }

    println!("Your hosts file is ready!");

    Ok(())
}

fn fetch_all_hosts(urls: &[&str], ignore_errors: bool) -> Result<Vec<String>, HcError> {
    let mut fetched_content = Vec::new();
    let mut errors = Vec::new();

    std::thread::scope(|s| {
        let mut handles = vec![];

        // Spawn all threads
        for uri in urls.iter() {
            let handle = s.spawn(move || fetch(uri));
            handles.push((uri, handle));
        }

        // Wait for all threads and collect results
        for (uri, handle) in handles {
            match handle.join() {
                Ok(Ok(body)) => {
                    println!("fetched '{}' successfully", uri,);
                    fetched_content.push(body);
                }
                Ok(Err(err)) => {
                    let err = HcError::Fetch {
                        url: uri.to_string(),
                        source: Box::new(err),
                    };
                    eprintln!("fetching '{}' failed: {}", uri, err);
                    errors.push(err);
                }
                Err(_) => {
                    let err = HcError::Thread(format!("Thread for {} panicked", uri));
                    eprintln!("{}", err);
                    errors.push(err);
                }
            }
        }
    });

    // Check for errors AFTER all threads complete
    if !errors.is_empty() && !ignore_errors {
        return Err(errors.into_iter().next().unwrap());
    }

    Ok(fetched_content)
}
