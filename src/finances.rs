use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct IncomeOrExpense {
    id: i32,
    name: String,
    amount: f32,
    // should be either income or expense
    type_of: String
}

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {

    let home: String = env::var("HOME").unwrap();
    let file_path: String = format!("{}/finances.json", &home);

    let var_path: String = match env::var("FINANCES_PATH") {
        Ok(v) => v,
        Err(_) => {
            match Path::new(&file_path).exists() {
                true => {
                    env::set_var("FINANCES_PATH", file_path);
                    format!("{}/finances.json", &home)
                }
                false => {
                    match File::create(&file_path) {
                        Ok(_) => {
                            env::set_var("FINANCES_PATH", file_path);
                            format!("{}/finances.json",&home)
                        },
                        Err(e) => panic!("Failed to create file: {}", e),
                    }
                }
            }
        }
    };
    
    Ok(())
}

pub fn add(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        panic!("No arguments provided");
    }

    let var_path: String = env::var("FINANCES_PATH").expect("Failed to get FINANCES_PATH, make sure to run $financers setup");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(var_path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut income_or_expense: Vec<IncomeOrExpense> = serde_json::from_str(&contents).unwrap();

    Ok(())
}
