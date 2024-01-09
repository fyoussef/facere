mod utils;

use std::{
    fs,
    io::{self, Write},
};

fn main() {
    utils::clear_terminal();
    print!("Please, enter your path: ");
    let _ = io::stdout().flush();

    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path).unwrap();

    let recipient = mount_dest(&path);

    utils::create_dirs(&recipient.path);

    let file_exists = utils::file_exists(&recipient.path);
    match file_exists {
        Err(_) => {
            println!("The file already exists. If you continue the existing file will be removed.")
        }
        _ => (),
    };

    let all = format!("{}/{}.{}", &recipient.path, &recipient.filename, "ts");

    fs::write(all, "").unwrap();
}

struct Recipient {
    filename: String,
    path: String,
}

fn mount_dest(path: &str) -> Recipient {
    let mut split_path: Vec<&str> = path.split('/').collect();
    let filename = split_path.pop().unwrap();

    Recipient {
        filename: filename.to_string(),
        path: path.to_string().replace(filename, ""),
    }
}
