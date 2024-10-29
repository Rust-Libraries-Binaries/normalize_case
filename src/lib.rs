/// Converts a string to snake_case.
pub fn to_snake_case(s: &str) -> String {
    s.chars()
        .flat_map(|c| if c.is_uppercase() {
            vec!['_', c.to_ascii_lowercase()]
        } else {
            vec![c]
        })
        .collect::<String>()
        .trim_start_matches('_')
        .to_string()
}

/// Converts a string to camelCase.
pub fn to_camel_case(s: &str) -> String {
    let mut capitalize = false;
    let mut result = String::new();
    let mut first_char = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(if first_char { c.to_ascii_lowercase() } else { c });
            first_char = false;
        }
    }

    result
}

/// Converts a string to PascalCase.
pub fn to_pascal_case(s: &str) -> String {
    to_camel_case(s).chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello world"), "hello world");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("Hello_World"), "helloWorld");
        assert_eq!(to_camel_case("hello world"), "helloWorld");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
    }
}
