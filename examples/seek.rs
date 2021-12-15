use std::io::{prelude::*, SeekFrom};
use std::fs::{OpenOptions};

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.txt")?;
    
    file.seek(SeekFrom::Start(10))?;

    let mut buf = b"hello world!\n".to_vec();
    file.read_to_end(&mut buf)?;
    println!("{:?}", String::from_utf8(buf.clone()));

    file.seek(SeekFrom::Start(10))?;

    file.write(&buf)?;

    Ok(())
}

// -> foo.txt

// [package]
// name = "rust-unix"
// version = "0.1.0"
// authors = ["alex <jipings@outlook.com>"]
// edition = "2018"

// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// [dependencies]
// libc = "0.2.109"