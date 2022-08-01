#![deny(clippy::pedantic)]

use error_stack::{Context, IntoReport, Result, ResultExt};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::{fs, io, process};
use structopt::StructOpt;

const COLOR_RED: &str = "\x1b[31m";
const COLOR_WHITE: &str = "\x1b[0m";

enum ExitCode {
    HashEqual = 0,
    HashNotEqual = 255,
}

enum CmpResult {
    Equal(Cmp),
    NotEqual(Cmp),
}

struct Cmp {
    msg: String,
    file_hash: String,
    expected_hash: String,
}
impl Cmp {
    fn print(&self, quiet: bool) {
        if quiet {
            self.print_quiet();
        } else {
            self.print_verbose();
        }
    }

    fn print_verbose(&self) {
        println!("{}", self.msg);
        println!("Found   :: {}", self.file_hash);
        println!("Expected:: {}", self.expected_hash);
    }

    fn print_quiet(&self) {
        println!("{}", self.file_hash);
        println!("{}", self.expected_hash);
    }
}

///  Calculate and compare hash values quickly and easily
#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
    expected_hash: String,
    /// Prints less information
    #[structopt(short, long)]
    quiet: bool,
}

#[derive(Debug)]
struct HashError;

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hash Error: Could not hash file...")
    }
}

impl Context for HashError {}

fn get_file_hash256(path: &Path) -> Result<String, HashError> {
    let mut hasher = Sha256::new();

    let mut file = fs::File::open(path)
        .report()
        .change_context(HashError)
        .attach_printable(format!("Could not open '{}'", path.display()))?;

    io::copy(&mut file, &mut hasher)
        .report()
        .change_context(HashError)
        .attach_printable(format!("Could not read from '{}'", path.display()))?;

    let hash256 = hasher.finalize();

    Ok(format!("{:x}", hash256))
}

fn hash_cmp(a: String, b: String) -> CmpResult {
    if a.len() != b.len() {
        return CmpResult::NotEqual(Cmp {
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
            cmp_marker = format!("{}{COLOR_RED}{}{COLOR_WHITE}", cmp_marker, b as char);
            is_equal = false;
        }
    }

    if is_equal {
        CmpResult::Equal(Cmp {
            msg: "Hashes are equal!".to_string(),
            file_hash: a,
            expected_hash: cmp_marker,
        })
    } else {
        CmpResult::NotEqual(Cmp {
            msg: "Hashes are not equal!".to_string(),
            file_hash: a,
            expected_hash: cmp_marker,
        })
    }
}

fn main() {
    let args = Args::from_args();

    let file_hash = match get_file_hash256(&args.input_file) {
        Ok(f) => f,
        Err(e) => panic!("{e:?}"),
    };

    match hash_cmp(file_hash, args.expected_hash) {
        CmpResult::Equal(cmp) => {
            cmp.print(args.quiet);
            process::exit(ExitCode::HashEqual as i32);
        }
        CmpResult::NotEqual(cmp) => {
            cmp.print(args.quiet);
            process::exit(ExitCode::HashNotEqual as i32);
        }
    }
}
