use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek};
use std::path::Path;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct IncomeOrExpense {
    id: i32,
    name: String,
    amount: f32,
    // should be either income or expense
    type_of: String
}

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("/home/chrisks/finances.json");

    match Path::new(&file_path).exists() {
        true => {
            format!("{}", &file_path)
        }
        false => {
            match File::create(&file_path) {
                Ok(_) => {
                    format!("{}",&file_path)
                },
                Err(e) => panic!("Failed to create file: {}", e),
            }
        }
    };
    
    Ok(())
}

pub fn add(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        panic!("No arguments provided");
    }

    if args.len() < 4 {
        panic!("Expected 4 arguments: Command, Name, Amount and Type");
    }

    let file_path = "/home/chrisks/finances.json";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut income_or_expense = match serde_json::from_str::<Vec<IncomeOrExpense>>(&contents) {
            Ok(v) => v,
            Err(e) => {
                if !contents.is_empty() {
                    return Err(format!(
                            "Error deserializing file contents: {:?}",
                            e.to_string()
                            )
                        .into());
                } else {
                    vec![]
                }
            }
        };

    let mut last_id = 0;

    if income_or_expense.is_empty() {
        print!("{}",1);

        let amount_from_args = args[2].clone();
        let amount = if let Ok(val) = amount_from_args.parse::<f32>() {
            val
        } else {
            panic!("Invalid amount provided: {}", amount_from_args);
        };

        let new_income_or_expense = IncomeOrExpense {
            id: last_id + 1,
            name: args[1].clone(),
            amount,
            type_of: args[3].clone()
        };

        let mut income_or_expense: Vec<IncomeOrExpense> = Vec::new();

        income_or_expense.push(new_income_or_expense);

        let income_or_expense_json = serde_json::to_string(&income_or_expense)?;

        // file.write_all(b"")?;
        file.write_all(income_or_expense_json.as_bytes())?;

        Ok(())
    } else {
        print!("{}",2);



        if income_or_expense.len() > 1 {
            if let Some(last) = income_or_expense.last_mut() {
                last_id = last.id;
            };
        };


        let amount_from_args = args[2].clone();
        let amount = if let Ok(val) = amount_from_args.parse::<f32>() {
            val
        } else {
            panic!("Invalid amount provided: {}", amount_from_args);
        };

        let new_income_or_expense = IncomeOrExpense {
            id: last_id + 1,
            name: args[1].clone(),
            amount,
            type_of: args[3].clone()
        };

        income_or_expense.push(new_income_or_expense);

        let income_or_expense_json = serde_json::to_string(&income_or_expense)?;

        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(income_or_expense_json.as_bytes())?;

        Ok(())
    }

}
