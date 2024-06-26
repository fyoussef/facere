mod utils;
mod recipient;
mod facere;

use std::process;
use clap::Parser;
use facere::{TemplateOptions, facere};
use recipient::Recipient;
use utils::{can_continue, file_exists};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Available templates to create file
    #[arg(value_enum)]
    template: TemplateOptions,

    /// Path to create file
    path: Option<String>,
}

fn main() {
    let input = Cli::parse();
    let template = input.template;
    let input_path = input.path.unwrap_or_else(|| {
        eprintln!("Invalid path provided");
        process::exit(1);
    });
    let recipient = Recipient::new(&input_path);
    let path_exists = file_exists(&recipient.full_path);
    if path_exists {
        let can_continue = can_continue();
        if can_continue {
            facere(&recipient, &template);
        } else {
            process::exit(1);
        }
    }
    facere(&recipient, &template);
}
