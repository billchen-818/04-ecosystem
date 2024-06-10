use anyhow::Context;
use ecosystem::MyError;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    println!(
        "size of std::io::Error: {}",
        std::mem::size_of::<std::io::Error>()
    );
    println!(
        "size of std::num::PaeseIntError: {}",
        std::mem::size_of::<std::num::ParseIntError>()
    );
    println!("size of String: {}", std::mem::size_of::<String>());
    println!(
        "size of serde-json: {}",
        std::mem::size_of::<serde_json::Error>()
    );
    println!("size of MyError is: {}", std::mem::size_of::<MyError>());

    let filename = "no-existent-file.txt";

    let _fd =
        fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("this is a custom error".to_string()))
}
