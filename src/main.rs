pub mod parser;
pub use parser::*;

// use clap::{Arg};
// use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use clap::Parser;

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

    match read_and_parse_yaml(&args.src, &args.out) {
        Ok(_) => {
            println!("✅ Successfully built Mocha test!");
        }
        Err(e) => eprintln!("Oops, looks like something went wrong: {}", e),
    }
}


fn read_and_parse_yaml(file_path: &str, out_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open and read the YAML file
    let mut file = File::open(file_path)?;
    let mut yaml_content = String::new();
    file.read_to_string(&mut yaml_content)?;

    // Parse YAML into your Config struct
    let config = LavaConfig::try_from(yaml_content.as_str())?;
    let mut file = File::create(out_path)?;
    file.write_all(config.to_mocha().as_bytes())?;
    Ok(())
}
