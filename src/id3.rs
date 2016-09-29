use tools;

use std::fs::*;
use std::io::Write;

#[allow(dead_code)]
pub struct ID3 {
    pub id3: bool,
    pub version: (u8, u8),

    pub artist: String,
    pub album: String,
    pub title: String,

    pub track: u8,

    pub folder_name: String,
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

        tools::copy_from_vec(&mut header, vec, 0, 3);
        tools::copy_from_vec(&mut artist, vec, size - 95, 30);
        tools::copy_from_vec(&mut album, vec, size - 65, 30);
        tools::copy_from_vec(&mut title, vec, size - 125, 30);

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