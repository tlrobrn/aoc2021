#![warn(clippy::all, clippy::pedantic)]
#![feature(stdin_forwarders)]
use std::io;
use std::result::Result;

pub fn input_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(Result::unwrap_or_default)
}
