# hash-cmp
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)\
Calculate and compare hash values quickly and easily

## Supported Hashing Algorithms
- Sha256

## How to use
```bash
./sha-cmp [optional: -q (quiet mode)] <file path> <expected hash>
```

## Exit codes
- **0**: Hashes are equal
- **255**: Hashes are not equal

## Example
### Verbose mode
![image](https://user-images.githubusercontent.com/77125551/181935043-354dbbfe-8d2d-4494-bd4b-b6786328c5e0.png)

### Quiet mode
![image](https://user-images.githubusercontent.com/77125551/181935057-8e907067-6e49-4664-87f7-89b72c9221ad.png)


## Roadmap
- Automatically figure out suitable hash-functions based on hash length
- Run trough multiple hash functions and describe differences
- optimized source code
- a man page

