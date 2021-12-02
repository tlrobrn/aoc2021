#![warn(clippy::all, clippy::pedantic)]
#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
}
