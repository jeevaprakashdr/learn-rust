#[cfg(test)]
mod tests {
    
    #[test]
    fn failure() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        }else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn success() {
        assert_eq!(2+2, 4);
    }
}