mod apientry;
mod commands;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use apientry::ApiEntry;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(file_path) = commands::get_file_path() {
        let json = load_file_contents(file_path)?;
        let entries = parse_json(json)?;
        let output = convert_entries_to_output(entries)?;

        if let Some(file_to_save) = commands::save_to_file() {
            save_file(file_to_save, output)?;
            return Ok(());
        }

        println!("{}", output);
        return Ok(());
    }

    commands::help();
    Ok(())
}

fn load_file_contents(file_path: String) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
        
    Ok(buffer)
}

fn save_file(file_path: String, content: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write(content.as_bytes())?;

    Ok(())
}

fn parse_json(data: String) -> Result<Vec<ApiEntry>, Box<dyn Error>> {
    let json: Value = serde_json::from_str(&data)?;

    let entries = match json.as_array() {
        Some(value) => value,
        None => return Err(Box::from("failed to read json array")),
    };

    let entries = entries
        .iter()
        .fold(Vec::new(), |mut list, entry| {
            if let Some(entry) = ApiEntry::from(entry) {
                list.push(entry);
            }
            list
        });

    Ok(entries)
}

fn convert_entries_to_output(entries: Vec<ApiEntry>) -> Result<String, Box<dyn Error>> {
    let mut table_of_content = entries
        .iter()
        .fold(String::new(), |mut value, entry| {
            let anchor = &entry.name.replace(" ", "-").to_lowercase();
            value.push_str(format!("* [{}](#{})\n", &entry.name, anchor).as_str());

            value
        });

    table_of_content.push_str("\n\n");

    let output = entries
        .iter()
        .fold(table_of_content, |mut output, entry| {
            output.push_str(&entry.generate_output());
            output
        });

    Ok(output)
}