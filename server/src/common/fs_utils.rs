use std::io::Read;
use std::fs::{File, metadata};

pub fn get_file_buffer(file_path: &str) -> Vec<u8> {    
    let mut file = File::open(file_path).expect("File was not found");
    let metadata = metadata(file_path).expect("File metadata not found");
    let mut file_buffer = vec![0; metadata.len() as usize];
    file.read(&mut file_buffer).expect("Buffer overflow");
    file_buffer
}