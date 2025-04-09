# Benchmarks between Rust and C++

## Setup

- Install `direnv`, `nix`, `rustup`.

Then run the following:
```bash
rustup default stable
direnv allow
```

## Run

In each directory, run

```
make run
```

3 times, and note the realtime time taken for Rust and C++.

### Get Permutations

Get all permutations of ABCDEFGHIJ

Results on my laptop:
```
Rust: Average 0.72s (0.72s, 0.71s, 0.72s)
C++: Average 0.56s (0.63s, 0.62s, 0.43s)
```
