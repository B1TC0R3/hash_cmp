use std::env;
use std::error::Error;

fn print_help() {
    println!("Usage: sha-cmp <method> <input-file> <sha>");
}

fn check_args(args: Vec<String>) -> Result<(), Box<dyn Error>>{
    if args.len() == 2 && args[1] == "-h" {
        print_help();
        return Ok(());

    } else if args.len() == 4 {
        return Ok(());

    }
    Err("Invalid Arguments!".into())
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    match check_args(args) {
        Err(e) => return Error(),
        _ => {}
    }

    Ok(())
}
