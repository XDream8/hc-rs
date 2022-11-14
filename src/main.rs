mod fetch;
mod rmd;
mod write;

use rmd::remove_duplicates;
use write::write;

// for cli-args
use seahorse::{App, Context,Flag, FlagType};
use std::env;

// colored output
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [args]", env!("CARGO_PKG_NAME")))
        .action(action)
        // flags
        .flag(
            Flag::new("urls", FlagType::String)
            .description("hosts urls(seperate them with spaces!)")
            .alias("u")
            )
        .flag(
            Flag::new("output", FlagType::String)
            .description("name of the output file")
            .alias("o")
            )
        .flag(
            Flag::new("rm_duplicate_lines", FlagType::Bool)
            .description("remove duplicate lines from the new hosts file")
            .alias("rmd")
        );

    app.run(args);
}

fn action(c: &Context) {
            // set default urls
            let urls = match c.string_flag("urls") {
                Ok(f) => f,
                _ => "https://badmojr.github.io/1Hosts/Pro/hosts.txt https://hosts.oisd.nl https://block.energized.pro/ultimate/formats/hosts https://raw.githubusercontent.com/notracking/hosts-blocklists/master/hostnames.txt https://raw.githubusercontent.com/jerryn70/GoodbyeAds/master/Hosts/GoodbyeAds.txt".to_string(),
            };

            // set default output filename
            let filename = match c.string_flag("output") {
                Ok(f) => f,
                _ => "hosts".to_string(),
            };

            let rm_duplicate_lines: bool = c.bool_flag("rm_duplicate_lines");

            // give info
            println!("Filename: {}, Remove Duplicates: {}", filename.blue(), format!("{}" ,rm_duplicate_lines).yellow());

            // fetch urls and write to file
            write(urls, &filename);

            // remove duplicates if -rmd flag is used
            if rm_duplicate_lines == true {
                println!("{}", "Removing duplicate lines".blue());
                remove_duplicates(&filename);
            }

            println!("{}", "Your hosts file is ready!".green().bold())
}
