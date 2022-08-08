# hash-cmp
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)\
Calculate and compare hash values quickly and easily.\
Varifying the integrity of files is somewhat tedious, as be default,
one has to calculate the file hash and then compare it to whatever they expected 
it to be manually.\
This tool makes this comparison a little more comfortable by *automatically calculating*
the hash value of a file, *comparing* it with the expected value and *highlighting* the differences in color.

Additionally, the tool supports multiple different hash functions.\
It will detect which hash method was used to create the expected hash value and decide
on the correct value to calculate autonomously.

## Supported Hashing Algorithms
- Sha224
- Sha256
- Sha384
- Sha512

## How to use
```bash
./hash-cmp -h
./hash-cmp [optional: -q (quiet mode)] <file path> <expected hash>
```

## Exit codes

**This section is currently outdated!**
This will be fixed soon.

- **0**: Hashes are equal
- **255**: Hashes are not equal
- **999**: Hashes are not equal in length

## Example
### Verbose mode
![image](https://user-images.githubusercontent.com/77125551/181935367-3dd1910d-70be-4a01-80c7-9bd893a4682e.png)

### Quiet mode
![image](https://user-images.githubusercontent.com/77125551/181935391-d6f9275c-9bd8-4af5-b302-f50bb641d7a3.png)


## Roadmap
- a AUR package
