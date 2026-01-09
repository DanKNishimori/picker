use std::{
    io::{self, Write},
    str::FromStr,
};

pub mod picker;

const HELP_MSG: &str = "!go or !draw
!quit or !exit
!load
";

pub fn run_repl() -> Result<(), io::Error> {
    let mut entries = String::new();
    println!("<< REPL Mode ; type !help for some advices >>\n");

    loop {
        let raw_entry = get_input(">>> ")?;
        let new_entry = raw_entry.trim();

        if new_entry.starts_with("!load") {
            let content = match std::fs::read_to_string(&new_entry[6..]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    continue;
                }
            };
            println!("loaded \"{}\".", &new_entry[6..]);
            entries.push_str(&content);
        } else if new_entry == "!help" {
            println!("{HELP_MSG}");
            continue;
        } else if new_entry == "!go" || new_entry == "!draw" {
            break;
        } else if new_entry == "!exit" || new_entry == "!quit" {
            return Ok(());
        } else {
            entries.push_str(new_entry);
        }
    }

    let picker = picker::Picker::from_str(&entries).expect("error parsing a float.");

    print_result(picker.draw());
    Ok(())
}

pub fn print_result(res: &str) {
    println!("The chosen was: {}", res)
}

fn get_input(msg: &str) -> io::Result<String> {
    print!("{msg}");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
