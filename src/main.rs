mod utils;
mod recipient;
mod facere;

use facere::TemplateOptions;
use recipient::Recipient;
use std::process;
use clap::Parser;

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
