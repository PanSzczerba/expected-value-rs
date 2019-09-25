# Expected Value
This program was made in order to help solve problem 8.4 from book *Programming Pearls* by Jon Bentley with the help of Rust programming language
## What does it do?
It generates random vectors sample of numbers from interval [-1, 1) and finds average maximal subvector sum.
## How to use it?
In order to compile it use:
```
cargo build
```
To count expected value of maximal subvector sum from vector length *n* with given *sample*:
```
./expected_value n sample
```
where **n** and **sample** are positive integers
