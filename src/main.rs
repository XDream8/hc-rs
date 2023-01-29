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

static HTTP_CLIENT: Lazy<reqwest::Client> =
    Lazy::new(|| reqwest::Client::builder().gzip(true).build().unwrap());

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
    let mut urls: Vec<String> = vec![];

    // take args and add them to urls vector
    for url in &c.args {
        urls.push(format!("{}", url).to_owned())
    }

    let default_hosts: Vec<&str> = vec![
        "https://badmojr.github.io/1Hosts/Pro/hosts.txt",
        "https://hosts.oisd.nl",
        "https://block.energized.pro/ultimate/formats/hosts",
        "https://raw.githubusercontent.com/notracking/hosts-blocklists/master/hostnames.txt",
        "https://raw.githubusercontent.com/jerryn70/GoodbyeAds/master/Hosts/GoodbyeAds.txt",
    ];

    let minimal_hosts: Vec<&str> = vec![
        "https://badmojr.github.io/1Hosts/Pro/hosts.txt",
        "https://block.energized.pro/ultimate/formats/hosts",
    ];

    // set default urls
    if urls.is_empty() {
        if minimal {
            minimal_hosts
                .iter()
                .for_each(|host| urls.push(host.to_string()));
        } else {
            default_hosts
                .iter()
                .for_each(|host| urls.push(host.to_string()));
        }
    }

    // set default output filename
    let filename = match c.string_flag("output") {
        Ok(f) => f,
        _ => "hosts".to_string(),
    };

    // -rmd arg
    let rm_duplicate_lines: bool = c.bool_flag("rm_duplicate_lines");

    // give info
    println!(
        "Filename: {}, Remove Duplicates: {}",
        filename.blue(),
        format!("{}", rm_duplicate_lines).yellow()
    );

    std::thread::scope(|s| {
        // create file
        let _file = File::create(&filename).expect("Error encountered while creating a file");

        let fname = &filename;

        // info
        println!("{}", "Starting downloads(threaded)".blue().bold());

        for uri in urls.iter() {
            s.spawn(move || {
                // this is where we store fetched content
                let mut body: String = String::new();

                // Fetch url and store the fetched content in body
                match fetch(&uri, &mut body) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Couldn't fetch url!\n{:?}", e);
                    }
                }

                // if body is not empty write to file
                if !body.is_empty() {
                    println!(
                        "{} ({}) {}",
                        "fetched".green().bold(),
                        uri.yellow(),
                        "successfully".green().bold()
                    );

                    let f = File::options().append(true).open(&fname);

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
    if rm_duplicate_lines == true {
        println!("{}", "Removing duplicate lines".blue());
        remove_duplicates(&filename);
    }

    println!("{}", "Your hosts file is ready!".green().bold())
}
