// Kevin Cantu
// hash function testing

use std;
use bench;

fn djb(&&s: str) -> uint {
    let u: uint = 5381u;
    for c: u8 in s { u *= 33u; u += c as uint; }
    ret u;
}

fn main () {
   let meow = bench::word_of_god();
   bench::hash_bench ("Benching djb...     ", djb, meow);
}


