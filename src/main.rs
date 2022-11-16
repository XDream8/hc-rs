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
        );

    app.run(args);
}

fn action(c: &Context) {
    // urls
    let mut urls: Vec<String> = vec![];

    // take args and add them to urls vector
    for url in &c.args {
        urls.push(format!("{}", url).to_owned())
    }

    // set default urls
    if urls.is_empty() {
        urls.push("https://badmojr.github.io/1Hosts/Pro/hosts.txt".to_owned());
        urls.push("https://hosts.oisd.nl".to_owned());
        urls.push("https://block.energized.pro/ultimate/formats/hosts".to_owned());
        urls.push(
            "https://raw.githubusercontent.com/notracking/hosts-blocklists/master/hostnames.txt"
                .to_owned(),
        );
        urls.push(
            "https://raw.githubusercontent.com/jerryn70/GoodbyeAds/master/Hosts/GoodbyeAds.txt"
                .to_owned(),
        );
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

    // fetch urls and write to file
    let mut n: u8 = 1;

    // create file
    let mut file = File::create(&filename).expect("Error encountered while creating a file");

    // this is where we store fetched content
    let mut body: String = String::new();

    for uri in urls {
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

    // remove duplicates if -rmd flag is used
    if rm_duplicate_lines == true {
        println!("{}", "Removing duplicate lines".blue());
        remove_duplicates(&filename);
    }

    println!("{}", "Your hosts file is ready!".green().bold())
}
