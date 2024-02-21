// use clap::{Arg};
// use serde::{Deserialize, Serialize};
use clap::Parser;
use lava_core::LavaConfig;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of lava.yaml file
    #[arg(short, long)]
    src: String,

    /// Output directory for mocha test
    #[arg(short, long)]
    out: String,
}

fn main() {
    let args = Args::parse();

    match read_and_parse_json(&args.src, &args.out) {
        Ok(_) => {
            println!("✅ Successfully built Mocha test!");
        }
        Err(e) => eprintln!("Oops, looks like something went wrong: {}", e),
    }
}

fn read_and_parse_json(file_path: &str, out_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open and read the YAML file
    let file = File::open(file_path)?;

    // Parse JSON into your Config struct
    let config: LavaConfig = serde_json::from_reader(file)?;
    println!("{:?}", config);
    let mut file = File::create(out_path)?;
    file.write_all(config.to_mocha().as_bytes())?;
    Ok(())
}
