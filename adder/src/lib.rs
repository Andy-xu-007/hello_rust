pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_contain_name(){
        let result = greeting("andy");
        assert!(
            result.contains("andya"),
            "result is '{}'",
            result
        );
    }
}

pub fn greeting(name: &str) -> String {
    format!("hello, {}", name)
}