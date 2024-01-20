mod utils;
mod recipient;
mod facere;

use recipient::Recipient;
use std::{
    env,
    process,
};

#[derive(Debug)]
struct Input {
    template: String,
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
    let recipient = Recipient::new(&input.path);
    let file_exists = utils::file_exists(&recipient.full_path);
    if file_exists {
        let can_continue = utils::can_continue();
        if can_continue {
            facere::facere(&recipient);
        } else {
            process::exit(1);
        }
    }
    facere::facere(&recipient);
}
