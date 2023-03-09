pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn raise_panic() {
    panic!("just being silly");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn failing_test() {
        assert_eq!(2, 4);        
    }

    #[test]
    fn assert() {
        assert!(false);
    }

    #[test]
    #[should_panic]
    fn test_panic_raised() {
        raise_panic();
    }
}
