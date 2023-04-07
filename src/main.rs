mod finances;
use std::env;

use crate::finances::{setup, add};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.get(0).map(|s| s.as_str()) {
        Some("setup") => setup().map_err(|e| format!("Error setting up: {}", e))?,
        // on this case arguments must be: Command, Name, Amount and Type
        Some("add") => add(args).map_err(|e| format!("Error adding to file: {}", e))?,
        Some(s) => panic!("Unknown command: {}", s),
        None => todo!(),
    };

    Ok(())
}
