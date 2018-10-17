use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("Could not create file.");

    file.write_all(b"This is my byte output")
        .expect("Cannot write to the file");
}
