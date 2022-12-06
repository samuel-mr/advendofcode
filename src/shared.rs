use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_file_as_string(path : &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    // Open (readonly)
    let mut file = match File::open(&path) { 
        Err(why) => panic!("No se puede abrir {display}. Motivo: {why}"),
        Ok(file) => file,
    };

    // Read in string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("no puede leer {display}. Motivo: {why}"),
        Ok(file) => println!("file was readed. Size: {file} bytes"),
    };

    return s;
}