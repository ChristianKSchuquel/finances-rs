mod finances;
#[macro_use] extern crate prettytable;
use std::env;

use crate::finances::{add, remove, list};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let home_dir = dirs::home_dir().unwrap();
    let file_path = format!("{}/.finances.json", home_dir.display());

    match args.get(0).map(|s| s.as_str()) {
        // on this case arguments must be: Command, Name, Amount
        Some("add") => add(args, file_path).map_err(|e| format!("Error adding to file: {}", e))?,
        Some("rm") => remove(args, file_path).map_err(|e| format!("Error removing from file: {}", e))?,
        Some("list") => list(file_path).map_err(|e| format!("Error listing from file: {}", e))?,
        Some(s) => panic!("Unknown command: {}", s),
        None => todo!(),
    };

    Ok(())
}
