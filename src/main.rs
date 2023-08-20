mod fetch;
mod rmd;

use fetch::fetch;
use rmd::remove_duplicates;

// for file operations
use std::{fs::File, io::Write};

// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::env;

// colored output
use colored::*;

use once_cell::sync::Lazy;

// http client
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

// reusable lazy initialized HTTP CLIENT
pub static HTTP_CLIENT: Lazy<Agent> = Lazy::new(|| {
    AgentBuilder::new()
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .build()
});

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [urls] [args]", env!("CARGO_PKG_NAME")))
        .action(action)
        // flags
        .flag(
            Flag::new("output", FlagType::String)
                .description("name of the output file")
                .alias("o"),
        )
        .flag(
            Flag::new("rm_duplicate_lines", FlagType::Bool)
                .description("remove duplicate lines from the new hosts file")
                .alias("rmd"),
        )
        .flag(
            Flag::new("minimal", FlagType::Bool)
                .description("create a minimal hosts file")
                .alias("m"),
        );

    app.run(args);
}

fn action(c: &Context) {
    // --minimal,-m
    let minimal: bool = c.bool_flag("minimal");

    // urls
    let urls: Vec<&str> = if !c.args.is_empty() {
        c.args.iter().map(|url| url.as_str()).collect()
    } else if minimal {
        // minimal hosts
        vec!["https://badmojr.github.io/1Hosts/Pro/hosts.txt"]
    } else {
        // default hosts
        vec![
            "https://badmojr.github.io/1Hosts/Pro/hosts.txt",
            "https://hosts.oisd.nl",
            "https://raw.githubusercontent.com/notracking/hosts-blocklists/master/hostnames.txt",
            "https://raw.githubusercontent.com/jerryn70/GoodbyeAds/master/Hosts/GoodbyeAds.txt",
        ]
    };

    // set default output filename
    let filename: String = c.string_flag("output").unwrap_or(String::from("hosts"));

    // -rmd arg
    let rm_duplicate_lines: bool = c.bool_flag("rm_duplicate_lines");

    // give info
    println!(
        "Filename: {}\nMinimal: {}\nRemove Duplicates: {}",
        filename.blue(),
        minimal.to_string().yellow(),
        rm_duplicate_lines.to_string().yellow()
    );

    std::thread::scope(|s| {
        // create file
        let _file = File::create(&filename).expect("Error encountered while creating a file");

        let fname: &String = &filename;

        // info
        println!("{}", "Starting downloads(threaded)".blue().bold());

        for uri in urls.iter() {
            s.spawn(move || {
                // this is where we store fetched content
                let body: String = match fetch(uri) {
                    Ok(f) => f,
                    Err(err) => {
                        eprintln!(
                            "{} ({}) {}: {}",
                            "fetching".red().bold(),
                            uri.yellow(),
                            "failed".red().bold(),
                            err.to_string().red().bold(),
                        );
                        String::new()
                    }
                };

                // if body is not empty write to file
                if !body.is_empty() {
                    println!(
                        "{} ({}) {}",
                        "fetched".green().bold(),
                        uri.yellow(),
                        "successfully".green().bold()
                    );

                    let f = File::options().append(true).open(fname);

                    // write to file
                    match f.expect("error").write_all(body.as_bytes()) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("Couldn't write fetched content to file!\n{:?}", e);
                        }
                    }
                }
            });
        }
    });

    // remove duplicates if -rmd flag is used
    if rm_duplicate_lines && urls.len() == 1 {
        println!(
            "{}",
            "We only fetched 1 hosts file, no need to remove duplicates".blue()
        );
    } else if rm_duplicate_lines {
        println!("{}", "Removing duplicate lines".blue());
        remove_duplicates(&filename);
    }

    println!("{}", "Your hosts file is ready!".green().bold())
}
