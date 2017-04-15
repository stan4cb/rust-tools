use std::fs::*;
use std::io::{Read, Write, Seek, SeekFrom};

use std::env;
use std::path::Path;

mod id3;
mod tools;

const MAX: usize = 30;

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        let param: String = args.nth(1).unwrap();
        let f_name = args.last().unwrap();

        match param.as_ref() {
            "folder" => {
                if Path::new(&f_name).is_dir() {
                    let x = tools::prompt("Edit All 'album' | 'artist' anything else to end : ");
                    let target_field = x.lines()
                        .nth(0)
                        .unwrap();

                    let io_string = tools::prompt("Enter new value : ");
                    let mut new_name: String = io_string.clone();

                    if io_string.len() > MAX {
                        let (x, _) = io_string.split_at(MAX);
                        new_name = String::from(x);
                    }

                    for song in analize_folder(&Path::new(&f_name)) {
                        let mut file = File::open(&song.folder_name).expect("file open error");
                        let mut whole_file = Vec::new();

                        let size = file.read_to_end(&mut whole_file).expect("file read error");

                        // wierd stuff -> input includes new_line
                        let mut bytes: Vec<u8> = new_name.lines()
                            .nth(0)
                            .unwrap()
                            .to_owned()
                            .clone()
                            .into_bytes();

                        while bytes.len() <= MAX {
                            // fill the MAX
                            bytes.push(' ' as u8);
                        }

                        let b_pos = match target_field {
                            "artist" => 95,
                            "album" => 65,
                            _ => panic!("album or artist"),
                        };

                        tools::copy_from_vecv(&mut whole_file,
                                              &bytes,
                                              0,
                                              bytes.len(),
                                              size - b_pos);

                        if tools::write_vec_u8(&song.folder_name, &whole_file) {
                            println!("Saved to {}", &song.folder_name);
                        }
                    }
                }
            }
            "file" => analize_file(Path::new(&f_name)).info(),
            _ => println!("Wrong param"),
        }
    } else {
        println!("Enter params : .exe (file | folder)  name");
    }
}

fn analize_folder(folder: &Path) -> Vec<id3::ID3> {
    let mut data: Vec<id3::ID3> = vec![];

    for music in read_dir(&folder).unwrap() {
        match music {
            Ok(m) => {
                let path = m.path();
                if path.extension().unwrap() == "mp3" {
                    let c_song = analize_file(path.as_path());
                    c_song.info();

                    if c_song.is_id3() {
                        data.push(c_song);
                    }
                }
            }
            Err(e) => println!("Error : {}", e),
        };
    }

    return data;
}

fn is_id3(f: &mut File) -> bool {
    match f.seek(SeekFrom::Start(0)) {
        Ok(_) => {
            let mut buffer: [u8; 3] = [0u8; 3];
            let id3_header: [u8; 3] = [73, 68, 51];

            match f.read_exact(&mut buffer) {
                Ok(_) => {
                    if buffer == id3_header {
                        return true;
                    }
                }
                Err(e) => println!("file read error {}", e),
            };
        }
        Err(e) => println!("seek error {}", e),
    }

    return false;
}

fn analize_file(file_name: &Path) -> id3::ID3 {
    let mut id3 = id3::ID3::new();

    match File::open(&file_name) {
        Ok(mut file) => {
            if is_id3(&mut file) {
                let mut x : [u8;2] = [0u8;2];
                file.read_exact(&mut x);
                id3.version = (x[0], x[1]);

                match file.seek(SeekFrom::End(-125)) {
                    Ok(res) => {
                        let mut a: [u8; 30] = [0u8; 30];

                        file.read_exact(&mut a);
                        id3.title = String::from_utf8(a.iter().cloned().collect()).unwrap();

                        file.read_exact(&mut a);
                        id3.artist = String::from_utf8(a.iter().cloned().collect()).unwrap();

                        file.read_exact(&mut a);
                        id3.album = String::from_utf8(a.iter().cloned().collect()).unwrap();
                    }
                    Err(e) => println!("File reading error {}", e),
                }
            }
        }
        Err(e) => println!("File error {}", e),
    }

    return id3;
}
