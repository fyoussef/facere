use std::{fs, fmt::Error};

fn main() {
    let test_path = "./my/path";

    create_dirs(test_path); 

    let file_exists = file_exists(test_path);
    match file_exists {
        Err(_) => println!("The file already exists. If you continue the existing file will be removed."),
        _ => ()
    };

    let all = format!("{}/{}", test_path, "text.ts");

    fs::write(all, "console.log()").unwrap();
}

fn create_dirs(path: &str) {
    let path_err = fs::create_dir_all(path);
    match path_err {
        Err(err) => println!("Errou ao criar os diretórios: {}", err),
        _ => ()
    };
}

fn file_exists(path: &str) -> Result<(), Error> {
    let exists = fs::metadata(path);
    match exists {
        Err(_) => panic!("The file <file_name> does not exists"),
        _ => Ok(())
    }
}