use std::{env::args, fs, str::FromStr};

use picker::picker::Picker;

fn main() {
    let mut was_gave_no_args = false;
    let arg = get_arg().unwrap_or_else(|| {
        was_gave_no_args = true;
        "--repl".to_string()
    });

    if &arg == "-r" || &arg == "--repl" {
        picker::run_repl().unwrap_or_else(|e| eprintln!("{e}"));
    } else {
        let content = fs::read_to_string(arg).expect("error on reading the entry.");
        let picker = Picker::from_str(&content).expect("error somewhere");

        picker::print_result(picker.draw());
    }

    if was_gave_no_args {
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("error on reading the pause.");
    }
}

fn get_arg() -> Option<String> {
    let mut args = args();
    args.next();

    args.next()
}
