// A Simple Hello World
// First thing I learned to write in rust
// This function returns the greeting; Hello, world!
// cerner_2tothe5th_2021
pub fn hello() -> String {
    ("Hello, world!").to_string()
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
