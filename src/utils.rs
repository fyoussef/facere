use std::fs;

pub fn create_dirs(path: &String) {
    let path_err = fs::create_dir_all(path);
    match path_err {
        Err(err) => println!("Errou ao criar os diretórios: {}", err),
        _ => (),
    };
}

pub fn file_exists(path: &String) -> bool {
    let exists = fs::metadata(path);
    match exists {
        Err(_) => false,
        Ok(_) => true,
    }
}
