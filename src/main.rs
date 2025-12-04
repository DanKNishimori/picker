use std::env::args;

use picker;

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
        println!("file: {arg}");
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
