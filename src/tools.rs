use std::fs::*;
use std::io::Write;

pub fn copy_from_vec(target: &mut [u8], source: &Vec<u8>, start: usize, count: usize) {
    for i in start..(start + count) {
        target[i - start] = source[i];
    }
}

pub fn copy_from_vecv(target: &mut Vec<u8>,
                      source: &Vec<u8>,
                      start: usize,
                      count: usize,
                      offset: usize) {
    for i in start..(start + count) {
        target[offset + i - start] = source[i];
    }
}

pub fn write_file(file_name: &String, vec: &Vec<u8>) -> bool {
    match File::create(&file_name) {
        Ok(mut file) => {
            let _ = file.write_all(vec.as_slice());
            return true;
        }
        Err(e) => {
            println!("File create error : {}", e);
            return false;
        }
    };
}