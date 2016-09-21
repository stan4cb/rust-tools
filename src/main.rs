use std::fs::*;
use std::io::Read;
use std::io::Write;

use std::env;
use std::path::Path;

#[allow(dead_code)]
struct ID3 {
    id3: bool,
    version: (u8, u8),

    artist: String,
    album: String,
    title: String,

    track: u8,

    folder_name: String,
}

const MAX: usize = 30;

fn main() {
    let mut args = env::args();
    let io = std::io::stdin();
    let mut stdout = std::io::stdout();

    if args.len() > 2 {
        let param: String = args.nth(1).unwrap();
        let f_name = args.last().unwrap();

        match param.as_ref() {
            "folder" => {
                if Path::new(&f_name).is_dir() {
                    let mut id3_vec: Vec<ID3> = vec![];

                    for music in read_dir(&f_name).unwrap() {
                        match music {
                            Ok(m) => {
                                let path = m.path();
                                if path.extension().unwrap() == "mp3" {
                                    let c_song = analize_file(path.as_path());
                                    c_song.info();

                                    if c_song.is_id3() {
                                        id3_vec.push(c_song);
                                    }
                                }
                            }
                            Err(e) => println!("Error : {}", e),
                        };
                    }

                    print!("Edit All album | artist anything else to end : ");

                    let mut io_string = String::new();

                    let _ = stdout.flush();
                    io.read_line(&mut io_string).unwrap();


                    let x = io_string.clone();
                    let target_field = x.lines()
                        .nth(0)
                        .unwrap();

                    io_string = String::new();
                    io.read_line(&mut io_string).unwrap();;

                    let mut new_name: String = io_string.clone();

                    if io_string.len() > MAX {
                        let (x, _) = io_string.split_at(MAX);
                        new_name = String::from(x);
                    }

                    for song in id3_vec {
                        match File::open(&song.folder_name) {
                            Ok(mut file) => {
                                let mut whole_file = Vec::new();
                                match file.read_to_end(&mut whole_file) {
                                    Ok(size) => {
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

                                        copy_from_vecv(&mut whole_file,
                                                       &bytes,
                                                       0,
                                                       bytes.len(),
                                                       size - b_pos);

                                        if write_file(&song.folder_name, &whole_file) {
                                            println!("Saved to {}", &song.folder_name);
                                        }
                                    }
                                    Err(e) => println!("File reading error {}", e),
                                }
                            }
                            Err(e) => println!("File error {}", e),
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

fn analize_file(file_name: &Path) -> ID3 {
    let mut id3 = ID3::new();

    match File::open(&file_name) {
        Ok(mut file) => {
            let mut whole_file = Vec::new();

            match file.read_to_end(&mut whole_file) {
                Ok(f_size) => {
                    id3.from_vec(&mut whole_file, f_size);
                    id3.folder_name = file_name.to_string_lossy().into_owned();
                }
                Err(e) => println!("File reading error {}", e),
            }
        }
        Err(e) => println!("File error {}", e),
    }

    return id3;
}

#[allow(dead_code)]
impl ID3 {
    pub fn new() -> ID3 {
        ID3 {
            version: (0, 0),
            id3: false,
            artist: String::new(),
            album: String::new(),
            title: String::new(),
            track: 0,
            folder_name: String::new(),
        }
    }

    pub fn info(&self) {
        println!("File -> {}", self.folder_name);
        print!("ID3  -> ");
        println!("V {}.{}", self.version.0, self.version.1);

        println!("\tArtist is -> {}", self.artist);
        println!("\tAlbum is  -> {}", self.album);
        println!("\tTitle is  -> {}", self.title);
        println!("\tTrack is  -> {}", self.track);
    }

    pub fn create_folders(&mut self) -> &mut ID3 {
        self.folder_name = format!("Music_Lib/{}/{}",
                                   self.artist.replace(" ", ""),
                                   self.album.replace(" ", ""));

        match create_dir_all(&self.folder_name) {
            Ok(_) => println!("Path created : {}", self.folder_name),
            Err(e) => println!("{}", e),
        }

        self
    }

    pub fn write_file(&self, vec: &Vec<u8>) {
        let file_name = format!("{}/{} {}.mp3",
                                self.folder_name,
                                self.track,
                                self.title.replace(" ", ""));

        match File::create(&file_name) {
            Ok(mut file) => {
                println!("File create done at : {}", &file_name);
                file.write_all(vec.as_slice()).unwrap();
            }
            Err(e) => println!("File create error : {}", e),
        }
    }

    pub fn from_vec(&mut self, vec: &mut Vec<u8>, size: usize) {
        let mut header = [0u8; 3];

        let mut artist = [0u8; 30];
        let mut album = [0u8; 30];
        let mut title = [0u8; 30];

        copy_from_vec(&mut header, vec, 0, 3);
        copy_from_vec(&mut artist, vec, size - 95, 30);
        copy_from_vec(&mut album, vec, size - 65, 30);
        copy_from_vec(&mut title, vec, size - 125, 30);

        self.id3 = String::from_utf8(header.iter().cloned().collect()).unwrap() == "ID3";
        self.version = (vec[3], vec[4]);
        self.track = vec[size - 2];

        self.artist = String::from_utf8(artist.iter().cloned().collect()).unwrap();
        self.album = String::from_utf8(album.iter().cloned().collect()).unwrap();
        self.title = String::from_utf8(title.iter().cloned().collect()).unwrap();
    }

    pub fn is_id3(&self) -> bool {
        self.id3
    }
}

fn copy_from_vec(target: &mut [u8], source: &Vec<u8>, start: usize, count: usize) {
    for i in start..(start + count) {
        target[i - start] = source[i];
    }
}

fn copy_from_vecv(target: &mut Vec<u8>,
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
