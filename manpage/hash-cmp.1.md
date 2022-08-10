% hash-cmp(1) hash-cmp 4.0.0
% Thomas Gingele (https://github.com/B1TC0R3)
% August 2022

# NAME
hash-cmp - check hash values easily

# SYNOPSIS
**hash-cmp** [*OPTION...*] input\_file expected\_hash

# DESCRIPTION
**hash-cmp** will help calculate and compare hash values quickly and easily.
This tool automatically calculates the hash value of a file, compares it with the expected value and highlights the differences in color.

Additionally, the tool supports multiple different hash functions.
It will detect which hash method was used to create the expected hash value and decide on the correct value to calculate autonomously.
Supported hash functions are:\
		- Sha224\
		- Sha256\
		- Sha384\
		- Sha512

# OPTIONS
**-h**
: display command usage

**-q**
: quiet mode, only print the hash values without any additional information

# EXIT CODES
**0**
: the calculated hash matches the expected hash

**300**
: Failed to detect hash method

**400**
: the calculated hash does NOT match the expected hash

# EXAMPLES
**hash-cmp -h**
: display command usage

**hash-cmp example.xml 8465e0df920dbedafbc82c1c8a4fe60d51df9d42d3cf05619b17d45b**
: check whether *example.xml* has the expected hash value

# COPYRIGHT
Copyright 2022 Thomas Gingele. License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>. This is free software: you are free to change and redistribute it. There is NO WARRANTY, to the extend permitted by law.
