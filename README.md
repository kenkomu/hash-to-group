# hash-to-group
# Hash-to-Group Function in Rust

This Rust project implements a hash-to-group function based on Pedersen's hash using the cyclic group \( \mathbb{Z}^*_5 \). This function uses SHA-256 to hash an input string and then maps it to an element in \( \mathbb{Z}^*_5 \) using predefined generators. The result is a secure way to map any binary input to a group element.

## Table of Contents
- [Overview](#overview)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Example Output](#example-output)
- [Explanation](#explanation)
- [License](#license)

## Overview

This project computes a hash-to-group function \( H_{2,3} \) that maps binary inputs to elements in the group \( \mathbb{Z}^*_5 \) (integers modulo 5). The function uses SHA-256 as a hash function to convert the input into a pair of bits, which are then used as exponents for the group's generators (2 and 3 in this case). The final value is the product of these generators raised to the extracted bit powers.

## Requirements

- Rust (1.56.0 or higher)
- `sha2` crate for SHA-256 hashing

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/hash-to-group.git
   cd hash-to-group
