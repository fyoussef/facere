use clap::ValueEnum;

use crate::{utils, recipient::Recipient};

use std::{fs, collections::HashMap, process};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TemplateOptions {
    /// Template to create class
    Class,
    /// Template to create class
    Cl,
    /// Template to create interfaces
    Interface,
    /// Template to create interfaces
    Itf
}

impl TemplateOptions {
    fn unwrap(&self) -> String {
        match &self {
            TemplateOptions::Cl | TemplateOptions::Class => "class".to_string(),
            TemplateOptions::Itf | TemplateOptions::Interface => "interface".to_string(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Template {
    pub name: String
}

impl Template {
    fn new(name: String) -> Template {
        Template { name }
    }

    pub fn get_template_content(name: String) -> String {
        let templs = HashMap::from([
            (
                Template::new("class".to_string()), 
                "export class ClassName {}".to_string()
            ),
            (
                Template::new("interface".to_string()), 
                "export interface IInterfaceName {}".to_string()
            )
        ]);
        let mut result = String::new();
        for (templ, content) in templs {
            if templ.name.eq(&name.to_lowercase()) {
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

pub fn facere(recipient: &Recipient, template_opt: &TemplateOptions) {
    let mut path = recipient.filename.to_owned();
    if !recipient.path.is_empty() {
        utils::create_dirs(&recipient.path);    
        path = format!("{}/{}", &recipient.path, &recipient.filename);
    }
    let content = Template::get_template_content(template_opt.unwrap());
    fs::write(path, content).unwrap();
}