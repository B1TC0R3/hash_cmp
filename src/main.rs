use std::{env, process, fs};
use std::error::Error;
use sha2::{Sha256, Digest};

fn print_help() {
    println!("Usage: sha-cmp <method> <input-file> <sha>");
}

fn parse_args(mut args: Vec<String>) -> Result<(String, String), Box<dyn Error>>{
    if args[1] == "-h" {
        print_help();
        process::exit(0);
    }

    if args.len() == 3 {
        return Ok((args.pop().unwrap(), 
                   args.pop().unwrap())); 
    }
    Err("Invalid Arguments!".into())
}

fn get_file_hash256(path: String) -> String {
    let mut hasher = Sha256::new();
    let mut file_content = fs::read_to_string(path).expect("Failed to read file");
    hasher.update(&mut file_content);
    let hash256 = hasher.finalize();

    format!("{:x}", hash256)
}

fn hash_cmp(a: String, b: String) -> (bool, String) {
    if a.len() != b.len() {
        return (false, "Hash values are not equal in length.".to_string());
    } else {
        let mut is_equal: bool = true;
        let mut cmp_marker: String = String::new();
        let mut cmp_char: char;

        for i in 0..a.len() {
            cmp_char = b.chars().nth(i).unwrap();
            if a.chars().nth(i).unwrap() != cmp_char {
                cmp_marker = format!("{}\x1b[31m{}\x1b[0m", cmp_marker, cmp_char);
                is_equal = false;
            } else {
                cmp_marker = format!("{}{}", cmp_marker, cmp_char);
            }
        }

        let result = format!("{}\n{}", a, cmp_marker);
        return (is_equal, result);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let file_hash: String;
    let expected_hash: String;

    match parse_args(args) {
        Err(e) => return Err(e),
        Ok((hash_a, hash_b)) => {
            file_hash = hash_a;
            expected_hash = hash_b;
        }
    }

    let (is_equal, msg) = hash_cmp(file_hash, expected_hash);
    if is_equal {
        println!("\x1b[32mThe hash values are equal.\x1b[0m");
    } else {
        println!("Found inconsistency:\n{}", msg);
    }

    Ok(())
}
