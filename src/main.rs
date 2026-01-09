fn main() {
    println!("Hello, Rust girl!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 3, 5);
    }
}
