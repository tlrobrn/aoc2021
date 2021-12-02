# Advent of Code 2021

[Advent of Code](https://adventofcode.com) solutions in [Rust](https://rust-lang.org).

## Structure

Common utilities are defined in [src/lib.rs](src/lib.rs).

Day implementations are defined in [src/bin/](src/bin/).

## Usage

Pass the day binary to `cargo run` and pass the input through `stdin`.
For example, to run `day1` with the input saved in `resources/day1.txt`:

```sh
$ cargo run --bin day1 < resources/day1.txt
```

## Test

```sh
$ cargo test
```

To test a specific day (e.g. `day1`):

```sh
$ cargo test --bin day1
```
