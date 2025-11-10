use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello")?;
    Ok(())
}

fn evaluate(value1: bool, value2: bool) -> bool {
    if value1 == value2 {
        return true;
    };
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_evaluate_true() {
        assert!(evaluate(true, true))
    }
    #[test]
    fn test_evaluate_false() {
        assert!(!evaluate(true, false))
    }
}
