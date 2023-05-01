mod finances;
#[macro_use] extern crate prettytable;
use std::env;

use crate::finances::{add, remove, list, clear};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let home_dir = dirs::home_dir().unwrap();
    let file_path = format!("{}/.finances.json", home_dir.display());

    match args.get(0).map(|s| s.as_str()) {
        // on this case arguments must be: Command, Name, Amount
        Some("add") => add(args, file_path).map_err(|e| format!("Error adding to file: {}", e))?,
        // on this case arguments must be: Index
        Some("rm") => remove(args, file_path).map_err(|e| format!("Error removing from file: {}", e))?,
        Some("list") => list(file_path).map_err(|e| format!("Error listing from file: {}", e))?,
        Some("clear") => clear(file_path).map_err(|e| format!("Error clearing file: {}", e))?,
        Some(s) => panic!("Unknown command: {}", s),
        None => print!("Finances-rs 
Usage: '$ fnc <command> [arguments]'

Commands:
    list:   Shows a table of the contents in the $HOME/.finances.json file
    add:    Adds an item to the finances file. Ex: '$ fnc add Income 20'
    rm:     Removes an item from the finances file. Ex: '$ fnc rm 1'
    clear:  Clears the finances file. Ex: '$ fnc clear'")
    };

    Ok(())
}
