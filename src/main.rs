use std::fs::File;
use std::io::Read;

use std::env;

struct ID3 {
    file_size : usize,
    id_header : [u8;3],

    title : [u8;30],
    artist : [u8;30],

    version : (u8, u8)
}

fn main() {
    let args = env::args();

    if args.len() > 0 {
        let file_name = args.last().unwrap();

        match File::open(&file_name)
        {
            Ok(mut file) => {
                let mut whole_file = Vec::new();

                match file.read_to_end(& mut whole_file){
                    Ok(f_size) => {
                        println!("File size for '{}' is : {} b", file_name ,f_size );

                        let mut id3 = ID3::new(f_size);

                        id3.from_vec(& mut whole_file);

                        //id3.from_file(& mut file);

                        if id3.is_id3() {
                            print!("ID3 -> ");
                            id3.info();
                        }
                        else {
                            println!("Not ID3");
                        }
                    },
                    Err(e) => println!("{}", e ),
                }
            },
            Err(e) => println!("{}" , e),
        }
    }
}

#[allow(dead_code)]
impl ID3 {
    pub fn new(file_size : usize) -> ID3 {
        ID3 { file_size : file_size ,id_header: [0u8;3] , title: [0u8;30], artist: [0u8;30], version : (0,0) }
    }

    pub fn info(&self) {
        println!("Version is {}.{}", self.version.0, self.version.1);

        println!("\tArtist is -> {}", String::from_utf8(self.artist.iter().cloned().collect()).unwrap());
        println!("\tTitle is -> {}", String::from_utf8(self.title.iter().cloned().collect()).unwrap());
    }

    pub fn from_vec(& mut self,vec: & mut Vec<u8>){
        for i in 0 .. 3 {
            self.id_header[i] = vec[i];
        }

        copy_from_vec(& mut self.id_header, vec, 0, 3);
        copy_from_vec(& mut self.title, vec, self.file_size - 125, 30);
        copy_from_vec(& mut self.artist, vec, self.file_size - 95, 30);

        self.version = (vec[3], vec[4]);
    }

    pub fn from_file(& mut self,file: & mut File) {
        let mut v_buff = [0u8;2];

        file.read_exact(& mut self.id_header).unwrap();
        file.read_exact(& mut v_buff).unwrap();

        self.version = (v_buff[0], v_buff[1]);
    }

    pub fn is_id3(& mut self) -> bool {
         String::from_utf8(self.id_header.iter().cloned().collect()).unwrap() == "ID3"
    }
}

fn copy_from_vec(target: & mut [u8], source: & Vec<u8>, start: usize, count: usize) {
    for i in start .. (start + count) {
        target[i - start] = source[i];
    }
}
