use std::io::{self, Write};

pub mod picker;

pub fn run_repl() -> Result<(), io::Error> {
    let mut entries = Vec::<String>::new();
    println!("<< REPL Mode ; list your entries then type \"!go\" or \"!draw\" >>\n");

    loop {
        let new_entry = get_input(">>> ")?;
        if new_entry.trim() == "!go" || new_entry.trim() == "!draw" {
            break;
        }
        entries.push(new_entry.trim().to_string());
    }

    let weights = (0..entries.len()).map(|_| 1f32).collect();
    let picker = picker::Picker::new(entries, weights);

    println!("{:?}", picker.draw());
    Ok(())
}

fn get_input(msg: &str) -> io::Result<String> {
    print!("{msg}");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
