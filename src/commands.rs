pub fn get_file_path() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return None,
    };

    if command != "--file" && command != "-f" {
        return None;
    }

    let value = match args.get(2) {
        Some(value) => value,
        None => return None,
    };

    if value.is_empty() {
        return None;
    }

    Some(value.to_owned())
}

pub fn save_to_file() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(3) {
        Some(value) => value,
        None => return None,
    };

    if command != "--save-to-file" && command != "-s" {
        return None;
    }

    let value = match args.get(4) {
        Some(value) => value,
        None => return None,
    };

    if value.is_empty() {
        return None;
    }

    Some(value.to_owned())
}

pub fn help() {
    println!("commands:");
    println!("\t-f\t--file\t\tPath to file to read");
    println!("\t-s\t--save-to-file\tIf this is set the output will be saved to a file instead of printed in the terminal");
}