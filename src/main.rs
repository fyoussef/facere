mod utils;

use clap::Parser;
use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
    process,
};

#[derive(Debug)]
struct Input {
    template: String,
    path: String,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Debug)]
struct Recipient {
    filename: String,
    path: String,
}

fn main() {
    let template = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Invalid template provided");
        process::exit(1);
    });
    let path = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Invalid path provided");
        process::exit(1);
    });
    let input = Input { template, path };
    let recipient = mount_recipient(&input.path);
    utils::create_dirs(&recipient.path);
    let file_exists = utils::file_exists(&recipient.path);
    match file_exists {
        Err(_) => {
            println!(
                "The file already exists. If you continue, the existing file will be overwritten."
            )
        }
        _ => pause(),
    };
    let all = format!("{}/{}.{}", &recipient.path, &recipient.filename, "ts");
    fs::write(all, "").unwrap();
}

fn mount_recipient(path: &str) -> Recipient {
    let mut split_path: Vec<&str> = path.split('/').collect();
    let filename = split_path.pop().unwrap();
    Recipient {
        filename: filename.to_string(),
        path: path.to_string().replace(filename, ""),
    }
}

fn pause() {
    let mut key_pressed = String::new();
    print!(
        "The file already exists. If you continue, the existing file will be overwritten. Are you sure to continue? [y/n] "
    );
    stdout().flush().unwrap();
    stdin().read_line(&mut key_pressed).unwrap();
    print!("pressionou: {key_pressed}");
}
