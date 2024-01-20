use crate::{utils, recipient::Recipient};

use std::fs;

pub fn facere(recipient: &Recipient) {
    utils::create_dirs(&recipient.path);
    let path = format!("{}/{}", &recipient.path, &recipient.filename);
    fs::write(path, "").unwrap();
}