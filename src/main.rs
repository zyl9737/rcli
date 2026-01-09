fn main() {
    println!("Hello, Rust boy!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 3, 5);
    }
}
