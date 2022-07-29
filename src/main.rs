use std::env;
use std::process;
use std::error::Error;

fn print_help() {
    println!("Usage: sha-cmp <method> <input-file> <sha>");
}

fn check_args(args: Vec<String>) -> Result<(), Box<dyn Error>>{
    if args[1] == "-h" {
        print_help();
        process::exit(0);
    }

    if args.len() == 3 {
        return Ok(());
    }
    Err("Invalid Arguments!".into())
}

fn hash_cmp(a: String, mut b: String) -> (bool, String) {
    if a.len() != b.len() {
        return (false, "Hash values are not equal in length.".to_string());
    } else {
        let mut is_equal: bool = true;
        for i in 0..a.len() {
            if a.chars().nth(i).unwrap() != b.chars().nth(i).unwrap() {
                is_equal = false;
            } 
        }
       
        let result = format!("{}\n{}", a, b);
    
        return (is_equal, result);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    match check_args(args) {
        Err(e) => return Err(e),
        _ => {}
    }

    Ok(())
}
