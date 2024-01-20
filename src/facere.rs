use crate::{utils, recipient::Recipient};

use std::{fs, collections::HashMap, process};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Template {
    pub name: String,
    pub short: String
}

impl Template {
    fn new(name: String, short: String) -> Template {
        Template { name, short }
    }

    pub fn get_template_content(name: String) -> String {
        let templs = HashMap::from([
            (
                Template::new("class".to_string(), "cl".to_string()), 
                "export class ClassName {}".to_string()
            ),
            (
                Template::new("interface".to_string(), "itf".to_string()), 
                "export interface IInterfaceName {}".to_string()
            )
        ]);
        let mut result = String::new();
        for (templ, content) in templs {
            if templ.name.eq(&name.to_lowercase()) || templ.short.eq(&name) {
                result = content;
                break;
            }
        }
        if result.is_empty() {
            eprintln!("Invalid template provided");
            process::exit(1);
        }
        result
    }
}

pub fn facere(recipient: &Recipient, template_name: &String) {
    utils::create_dirs(&recipient.path);
    let path = format!("{}/{}", &recipient.path, &recipient.filename);
    let content = Template::get_template_content(template_name.to_owned());
    fs::write(path, content).unwrap();
}