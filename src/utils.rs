use std::{fmt::Error, fs};

pub fn create_dirs(path: &str) {
    let path_err = fs::create_dir_all(path);
    match path_err {
        Err(err) => println!("Errou ao criar os diretórios: {}", err),
        _ => (),
    };
}

pub fn file_exists(path: &str) -> Result<(), Error> {
    let exists = fs::metadata(path);
    match exists {
        Err(_) => panic!("The file <file_name> does not exists"),
        _ => Ok(()),
    }
}
