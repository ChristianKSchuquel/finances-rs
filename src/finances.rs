use std::fs::OpenOptions;
use std::io::{Read, Write, Seek};
use serde::{Serialize, Deserialize};
use prettytable::Table;


#[derive(Debug, Serialize, Deserialize)]
struct IncomeOrExpense {
    name: String,
    amount: f32,
}

pub fn add(args: Vec<String>, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 || args.len() > 3 {
        panic!("Expected 3 arguments: Command, Name and Amount");
    }


    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut income_or_expense = match serde_json::from_str::<Vec<IncomeOrExpense>>(&contents) {
        Ok(v) => v,
        Err(e) => {
            if !contents.is_empty() {
                return Err(format!("Error deserializing file contents: {:?}",e.to_string()).into());
            } else {
                vec![]
            }
        }
    };

    let amount_from_args = args[2].clone();
    let amount = if let Ok(val) = amount_from_args.parse::<f32>() {
        val
    } else {
        panic!("Invalid amount provided: {}", amount_from_args);
    };

    let new_income_or_expense = IncomeOrExpense {
        name: args[1].clone(),
        amount,
    };

    income_or_expense.push(new_income_or_expense);

    let income_or_expense_json = serde_json::to_string(&income_or_expense)?;

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(income_or_expense_json.as_bytes())?;

    Ok(())
}

pub fn remove(args: Vec<String>, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 || args.len() > 2 {
        panic!("Expected index");
    }


    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

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
    
    let mut index = args[1].parse::<usize>()?;

    if income_or_expense.is_empty() || income_or_expense.len() < index {
        panic!("Invalid id provided");
    };

    index = index - 1;

    income_or_expense.remove(index);

    let income_or_expense_json = serde_json::to_string(&income_or_expense)?;

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(income_or_expense_json.as_bytes())?;

    Ok(())
}

pub fn list(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let income_or_expense = match serde_json::from_str::<Vec<IncomeOrExpense>>(&contents) {
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

    let mut table = Table::new();
    table.add_row(row![Fyb => "index", "name", "value"]);

    let mut index = 1;

    for x in &income_or_expense {
        table.add_row(row![Fbb => index, x.name, x.amount]);
        index = index + 1;
    };

    let mut total: f32 = 0.0;
    for x in income_or_expense {
        total = total + x.amount;
    };

    match total {
        x if x > 0.0 => {
            table.add_row(row![Fgb => "", "Total", total]);
        }
        x if x < 0.0 => {
            table.add_row(row![Frb => "", "Total", total]);
        }
        _ => {
            table.add_row(row![Fwb => "", "Total", total]);
        }
    }

    table.printstd();

    Ok(())
}

pub fn clear(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    Ok(())
}
