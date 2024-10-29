# normalize-case

A small, dependency-free Rust library for converting strings to various cases: `snake_case`, `camelCase`, and `PascalCase`.

## Overview

`normalize-case` provides simple and efficient functions to convert string cases, ideal for Rust applications that require case normalization for structured data, configuration keys, or general text formatting.

## Installation

Add `normalize-case` to your `Cargo.toml`:

```toml
[dependencies]
normalize-case = "0.1.0"```

## Usage

use normalize_case::{to_snake_case, to_camel_case, to_pascal_case};

fn main() {
    assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(to_camel_case("hello_world"), "helloWorld");
    assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
}
Available Functions
to_snake_case(s: &str) -> String: Converts a string to snake_case.
to_camel_case(s: &str) -> String: Converts a string to camelCase.
to_pascal_case(s: &str) -> String: Converts a string to PascalCase.
Contributing
Contributions are welcome! Please feel free to submit a pull request or report issues.

## License
This project is licensed under the MIT License.