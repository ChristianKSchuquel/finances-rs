use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use rand::Rng;
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

    match env::var("FINANCES_PATH") {
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

pub fn add(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        panic!("No arguments provided");
    }

    if args[3].to_lowercase() != "expense" || args[3].to_lowercase() != "income" {
        panic!("Invalid type, should be either 'expense' or 'income'")
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

    let mut incomes_or_expenses: Vec<IncomeOrExpense> = serde_json::from_str(&contents).unwrap();

    let mut new_income_or_expense = IncomeOrExpense {
        id: 1,
        name: args[1].clone(),
        amount: args[2].parse::<f32>().unwrap(),
        type_of: args[3].clone()
    };

    if let Some(last_item) = incomes_or_expenses.last() {
        new_income_or_expense.id = last_item.id + new_income_or_expense.id;   
    }

    incomes_or_expenses.push(new_income_or_expense);

    let new_contents = serde_json::to_string(&incomes_or_expenses).unwrap();

    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write_all(new_contents.as_bytes()).unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
