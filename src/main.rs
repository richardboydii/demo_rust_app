fn main() {
    println!("Hello, SWA!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        main()
    }
}