use std::fs::File;
use std::io;
use std::io::{Read, Write};

pub fn read_file_as_str(file: &str) -> String {
    read_file_contents(file).unwrap()
}
pub fn read_file_contents(file: &str) -> io::Result<String> {
    let mut file_data = File::open(&file)?;
    let mut text = String::new();
    file_data.read_to_string(&mut text)?;
    Ok(text)
}


// pub fn write_to_file(name : &str, content : &str) {
//     let mut file = File::create(name)?;
//     file.write_all(content.encode_utf16().)?;
// }
