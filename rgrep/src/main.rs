use rgrep::search;
use std::{env, fs, process};

fn main() {
    let mut args = env::args().skip(1);
    let Some(query) = args.next() else {
        print_usage_and_exit();
    };
    let Some(path) = args.next() else {
        print_usage_and_exit();
    };

    if args.next().is_some() {
        print_usage_and_exit();
    }

    let contents = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("error: could not read {path}: {error}");
        process::exit(1);
    });

    for line in search(&query, &contents) {
        println!("{line}");
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: rgrep <query> <file>");
    process::exit(2);
}
