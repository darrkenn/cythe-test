use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello")?;
    Ok(())
}
