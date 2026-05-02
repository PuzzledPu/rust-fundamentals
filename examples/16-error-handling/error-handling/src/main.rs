use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <filename>", env::args().next().unwrap_or_else(|| "file-reader".into()));
        std::process::exit(1);
    });

    let file = File::open(&filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
