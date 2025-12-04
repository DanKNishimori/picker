use std::io::{self, Write};

pub fn run_repl() -> Result<(), io::Error> {
    let mut entries = Vec::<String>::new();
    println!("<< REPL Mode ; list your entries then type \"!go\" or \"!draw\" >>\n");

    loop {
        let new_entry = get_input(">>> ").unwrap();
        if new_entry.trim() == "!go" || new_entry.trim() == "!draw" {
            break;
        }
        entries.push(new_entry.trim().to_owned());
    }

    println!("{entries:?}");
    Ok(())
}

fn get_input(msg: &str) -> io::Result<String> {
    print!("{msg}");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
