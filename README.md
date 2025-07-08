# Rust Algorithm

A project for learning and practicing Rust algorithms, including LeetCode solutions and algorithm book implementations.

## Project Structure

```
rust_algorithm/
├── book/           # Algorithm book implementations
│   └── src/
│       ├── chapter/    # Chapter code
│       │   ├── c12.rs  # Chapter 12
│       │   ├── c13.rs  # Chapter 13
│       │   └── c15.rs  # Chapter 15
│       └── main.rs
├── leetcode/       # LeetCode solutions
│   └── src/
│       ├── question/   # Problem solutions
│       │   ├── w4.rs   # Problem 4
│       │   ├── w10.rs  # Problem 10
│       │   ├── w23.rs  # Problem 23
│       │   ├── w25.rs  # Problem 25
│       │   └── w30.rs  # Problem 30
│       └── main.rs
└── src/
    └── main.rs     # Main entry point
```

## Usage

### Run the entire project
```bash
cargo run
```

### Run specific modules
```bash
# Run LeetCode solutions
cargo run -p leetcode

# Run book implementations
cargo run -p book
```

### Run tests
```bash
cargo test
```

### Build the project
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

## Dependencies

- Rust 2024 edition
- Internal dependencies:
  - `leetcode` - LeetCode solutions module
  - `book` - Algorithm book learning module

## Contributing

Pull requests and issues are welcome!