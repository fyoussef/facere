use std::{fs, io::{stdin, stdout, Write}};

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

pub fn can_continue() -> bool {
    let mut key_pressed = String::new();
    print!(
        "The file already exists. If you continue, the existing file will be overwritten. Are you sure to continue? [y/n] "
    );
    stdout().flush().unwrap();
    stdin().read_line(&mut key_pressed).unwrap();
    let ok = key_pressed.contains("y");
    ok
}
