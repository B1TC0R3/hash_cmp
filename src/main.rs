use std::{env, process, fs};
use std::error::Error;
use sha2::{Sha256, Digest};

enum ExitCode {
    HashEqual = 0,
    HashNotEqual = 99,
}

struct CmpResult {
    msg: String,
    file_hash: String,
    expected_hash: String,
}

fn print_help() {
    println!("Usage: hash-cmp [optional: -q] <input-file> <expected hash>");
    println!("Parameters:");
    println!("  -q: quiet mode");
}

fn print_verbose(cmp_result: CmpResult) {
    println!("{}", cmp_result.msg);
    println!("Found   :: {}", cmp_result.file_hash);
    println!("Expected:: {}", cmp_result.expected_hash);
}

fn print_quiet(cmp_result: CmpResult) {
    println!("{}", cmp_result.file_hash);
    println!("{}", cmp_result.expected_hash);
}

fn parse_args(mut args: Vec<String>) -> Result<(String, String), Box<dyn Error>>{
    if args[1] == "-h" {
        print_help();
        process::exit(0);
    }

    if args.len() >= 3 {
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

fn hash_cmp(a: String, b: String) -> Result<CmpResult, CmpResult> {
    if a.len() != b.len() {
        Err(CmpResult {
            msg: "Hash lengths do not match!".to_string(),
            file_hash: a,
            expected_hash: b,
        })
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

        if is_equal {
            Ok(CmpResult {
                msg: "Hashes are equal!".to_string(),
                file_hash: a,
                expected_hash: b,
            })
        } else {
            Err(CmpResult {
                msg: "Hashes are not equal!".to_string(),
                file_hash: a,
                expected_hash: b,
            })
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let file_hash: String;
    let expected_hash: String;

    match parse_args(args) {
        Err(e) => return Err(e),
        Ok((hash, file_path)) => {
            file_hash = get_file_hash256(file_path);
            expected_hash = hash;
        }
    }

    match hash_cmp(file_hash, expected_hash) {
        Ok(msg) => {
            match env::var("-q") {
                Ok(_) => print_quiet(msg),
                Err(_) => print_verbose(msg),
            }
            process::exit(ExitCode::HashEqual as i32);
        },
        Err(msg) => {
            match env::var("-q") {
                Ok(_) => print_quiet(msg),
                Err(_) => print_verbose(msg),
            }
            process::exit(ExitCode::HashNotEqual as i32);
        }
    }
}
