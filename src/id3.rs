use std::fs::File;
use std::io::Read;

use std::env;

fn main()
{
    let args = env::args();

    if args.len() > 0
    {
        let file_name = args.last().unwrap();

        match File::open(file_name) {
            Ok(mut file) => print!("Is it ID3 file -> {}", is_id3(&mut file)),
            Err(e) => println!("{}" , e),
        }
    }
}

fn is_id3(file: & mut File) -> bool
{
    let mut buf = [0u8;3];

    return match file.read_exact(&mut buf)
    {
        Ok(_) => String::from_utf8(buf.iter().cloned().collect()).unwrap() == "ID3",
        Err(_) => false,
    };
}
