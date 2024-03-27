use std::io::Read;

pub mod clap;
pub mod color;
pub mod udp;

pub fn stdin_to_string() -> String {
    let mut string = String::new();

    std::io::stdin()
        .read_to_string(&mut string)
        .expect("read standard input failed");

    string
}
