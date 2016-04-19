use std::fs::File;
use std::io::Read;

use std::env;

fn main()
{
    let args = env::args();

    if args.len() > 0
    {
        let file_name = args.last().unwrap();

        let mut id3 = ID3::new();


        match File::open(file_name)
        {
            Ok(mut file) =>
            {
                    id3.fill_from_file(& mut file);
                    println!("Is it ID3 file -> {}", id3.is_id3());
                    println!("Version is {:?}", id3.version );
            },
            Err(e) => println!("{}" , e),
        }
    }
}

struct ID3 {
    id_header : [u8;3],
    version : (u8, u8),
}

impl ID3 {
    pub fn new() -> ID3
    {
        ID3 {id_header: [0u8;3] , version : (0,0) }
    }

    pub fn fill_from_file(& mut self,file: & mut File)
    {
        let _ = file.read_exact(& mut self.id_header);

        let mut v_buff = [0u8;2];

        let _ = file.read_exact(& mut v_buff);

        self.version = (v_buff[0], v_buff[1]);
    }

    pub fn is_id3(& mut self) -> bool
    {
         String::from_utf8(self.id_header.iter().cloned().collect()).unwrap() == "ID3"
    }
}
