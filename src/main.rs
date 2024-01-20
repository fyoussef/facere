mod utils;
mod recipient;
mod facere;

use recipient::Recipient;
use std::process;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Input {
    template: Option<String>,
    path: Option<String>,
}

fn main() {
    let input = Input::parse();
    let template = input.template.unwrap_or_else(|| {
        eprintln!("Invalid template provided");
        process::exit(1);
    });
    let path = input.path.unwrap_or_else(|| {
        eprintln!("Invalid path provided");
        process::exit(1);
    });
    let recipient = Recipient::new(&path);
    let file_exists = utils::file_exists(&recipient.full_path);
    if file_exists {
        let can_continue = utils::can_continue();
        if can_continue {
            facere::facere(&recipient, &template);
        } else {
            process::exit(1);
        }
    }
    facere::facere(&recipient, &template);
}
