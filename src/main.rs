#![deny(clippy::pedantic)]

use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};
use std::error::Error;
use std::{env, fs, io, process};

/*
 * Hash legths:
 * - SHA-224: 56 bytes
 * - SHA-256: 64 bytes
 * - SHA-384: 96 bytes
 * - SHA-512: 128 bytes
 * */

enum ExitCode {
    HashEqual = 0,
    HashNotEqual = 255,
}

enum HashType {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Unknown,
}

enum CmpResult {
    Equal(CmpData),
    NotEqual(CmpData),
}

struct CmpData {
    msg: String,
    file_hash: String,
    expected_hash: String,
}

struct AutoSha {
    hash_type: HashType,
}

impl AutoSha {
    fn new(hash_size: usize) -> AutoSha{
        AutoSha {
            hash_type: match hash_size {
                56 => HashType::Sha224,
                64 => HashType::Sha256,
                96 => HashType::Sha384,
                128 => HashType::Sha512,
                _ => HashType::Unknown
            }
        }
    }

    fn get_hash(&self, file_path: String) -> String {
        let hash_result = match self.hash_type {
            HashType::Sha224 => self.calc_hash::<Sha224>(file_path),
            HashType::Sha256 => self.calc_hash::<Sha256>(file_path),
            HashType::Sha384 => self.calc_hash::<Sha384>(file_path),
            HashType::Sha512 => self.calc_hash::<Sha512>(file_path),
            HashType::Unknown => panic!("Expected hash is of unknown type.")
        };

        let hash = match hash_result {
            Ok(val) => val,
            Err(_) => panic!("Hash does not match known hash function.")
        };

        format!{"{}", hash}
    }

    fn calc_hash<T: Digest + io::Write>(&self, file_path: String) -> Result<String, Box<dyn Error>> {
        let mut hasher = T::new();
        let mut file = fs::File::open(file_path)?;

        io::copy(&mut file, &mut hasher)?;

        let hash = hasher.finalize();
        let mut hash_string = String::new();
        for byte in hash.iter() {
            hash_string = format!("{}{:02x}", hash_string, byte);
        }
        Ok(hash_string)
    }
}

fn print_help() {
    println!("Usage: hash-cmp [optional: -q] <input-file> <expected hash>");
    println!("Parameters:");
    println!("  -q: quiet mode");
}

fn print_verbose(cmp_data: &CmpData) {
    println!("{}", cmp_data.msg);
    println!("Found   :: {}", cmp_data.file_hash);
    println!("Expected:: {}", cmp_data.expected_hash);
}

fn print_quiet(cmp_data: &CmpData) {
    println!("{}", cmp_data.file_hash);
    println!("{}", cmp_data.expected_hash);
}

fn parse_args(mut args: Vec<String>) -> Result<(String, String, bool), Box<dyn Error>> {
    let mut quiet = false;

    if args.len() == 1 {
        print_help();
        process::exit(1);
    }

    if args[1] == "-h" {
        print_help();
        process::exit(0);
    }

    for arg in &args {
        if arg == "-q" {
            quiet = true;
        }
    }

    if args.len() >= 3 {
        return Ok((args.pop().unwrap(), args.pop().unwrap(), quiet));
    }
    Err("Invalid Arguments!".into())
}

fn hash_cmp(a: String, b: String) -> CmpResult {
    if a.len() != b.len() {
        return CmpResult::NotEqual(CmpData {
            msg: "Hash lengths do not match!".to_string(),
            file_hash: a,
            expected_hash: b,
        });
    }

    let mut is_equal: bool = true;
    let mut cmp_marker: String = String::new();

    for (a, b) in a.bytes().zip(b.bytes()) {
        if a == b {
            cmp_marker = format!("{}{}", cmp_marker, b as char);
        } else {
            cmp_marker = format!("{}\x1b[31m{}\x1b[0m", cmp_marker, b as char);
            is_equal = false;
        }
    }

    if is_equal {
        CmpResult::Equal(CmpData {
            msg: "Hashes are equal!".to_string(),
            file_hash: a,
            expected_hash: cmp_marker,
        })
    } else {
        CmpResult::NotEqual(CmpData {
            msg: "Hashes are not equal!".to_string(),
            file_hash: a,
            expected_hash: cmp_marker,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let auto_sha: AutoSha;
    let file_hash: String;
    let expected_hash: String;
    let quiet: bool;

    match parse_args(args) {
        Err(e) => return Err(e),
        Ok((hash, file_path, mode)) => {
            auto_sha = AutoSha::new(hash.len());
            file_hash = auto_sha.get_hash(file_path);
            expected_hash = hash;
            quiet = mode;
        }
    }

    match hash_cmp(file_hash, expected_hash) {
        CmpResult::Equal(data) => {
            match quiet {
                true => print_quiet(&data),
                false => print_verbose(&data),
            }
            process::exit(ExitCode::HashEqual as i32);
        }
        CmpResult::NotEqual(data) => {
            match quiet {
                true => print_quiet(&data),
                false => print_verbose(&data),
            }
            process::exit(ExitCode::HashNotEqual as i32);
        }
    }
}
