use std::{
    collections::BTreeSet,
    fs::File,
    io::{Write, BufRead, BufReader},
};

fn main() {
    let mut body: String = String::new();
    let urls = vec!("https://badmojr.github.io/1Hosts/Pro/hosts.txt",
                "https://hosts.oisd.nl",
                "https://block.energized.pro/ultimate/formats/hosts",
                "https://raw.githubusercontent.com/notracking/hosts-blocklists/master/hostnames.txt",
                "https://raw.githubusercontent.com/jerryn70/GoodbyeAds/master/Hosts/GoodbyeAds.txt",
                );

    let mut n: u8 = 1;
    let mut file = File::create("hosts").expect("Error encountered while creating a file");
    for uri in urls.iter() {
        println!("{}) {}", n, uri);
        // fetch url
        fetch(uri, &mut body);
        // write to file
        file.write_all(body.as_bytes());
        n += 1;
    }

    // remove duplicates
    println!("Removing duplicate lines");
    // open file
    let file = File::open("hosts").expect("Couldn't open file");
    let reader = BufReader::new(file);
    // read all uniq lines
    let lines: BTreeSet<_> = reader.lines()
        .map(|l| l.expect("Couldn't read a line"))
        .collect();

    let mut file = File::create("hosts").expect("Error encountered while creating a file");
    for line in lines {
        // write uniq line
        file.write_all(line.as_bytes())
            .expect("Couldn't write to file");

        // write newline(\n)
        file.write_all(b"\n").expect("Couldn't write to file")
    }

    println!("Your hosts file is ready!")
}

#[tokio::main]
async fn fetch(uri: &str, body: &mut String) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(uri).await?;
    *body = resp.text().await?;
    Ok(())
}
