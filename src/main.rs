use std::{env::args, fs, str::FromStr};

use picker::picker::Picker;

fn main() {
    let arg = match get_arg() {
        Ok(a) => a,
        Err(_) => {
            eprintln!("No argument was given! Type \"picker -h\" to get help.");
            std::process::exit(1);
        }
    };

    if &arg == "-r" || &arg == "--repl" {
        picker::run_repl().unwrap_or_else(|e| eprintln!("{e}"));
    } else {
        let content = fs::read_to_string(arg).expect("error on reading the entry.");
        let picker = Picker::from_str(&content).expect("error somewhere");

        for _ in 0..1 {
            println!("drawed: {}", picker.draw());
        }

        // std::io::stdin()
        //     .read_line(&mut String::new())
        //     .expect("error on reading the pause.");
    }
}

fn get_arg() -> Result<String, ()> {
    let mut args = args();
    args.next();

    match args.next() {
        Some(a) => Ok(a),
        None => Err(()),
    }
}
