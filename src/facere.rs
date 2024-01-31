use clap::ValueEnum;

use crate::{utils, recipient::Recipient};

use std::{collections::HashMap, fs, process};

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

    pub fn get_template_content(name: String, input: String) -> String {
        let templs = HashMap::from([
            (
                Template::new("class".to_string()), 
                format!("export class {input} {{}}").to_string()
            ),
            (
                Template::new("interface".to_string()), 
                format!("export interface I{input} {{}}").to_string()
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
    let input = format_input(recipient);
    let content = Template::get_template_content(template_opt.unwrap(), input);
    fs::write(path, content).unwrap();
}

fn format_input(recipient: &Recipient) -> String {
    let filename_without_ext = recipient.filename.replace(".ts", "");
    let splited: Vec<String> = filename_without_ext.trim().split("-").map(|x| x.to_string()).collect();
    let mut result = String::new();
    for split in splited {
        let mut chunk: Vec<String> = split.split("").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();
        chunk[0] = chunk[0].to_uppercase();
        println!("{:?}", chunk[0]);
        let joined = chunk.join("");
        result += &joined;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sholuld_return_template_class() {
        let expect = TemplateOptions::unwrap(&self::TemplateOptions::Cl);
        assert_eq!(expect, "class");
    }

    #[test]
    fn it_sholuld_return_template_interface() {
        let expect = TemplateOptions::unwrap(&self::TemplateOptions::Itf);
        assert_eq!(expect, "interface");
    }

    #[test]
    fn it_should_get_template() {
        let expect = Template::get_template_content("class".to_string(), "test".to_string());
        assert_eq!(expect, "export class test {}");
    }

    #[test]
    fn it_should_capitalize_string() {
        let input = Recipient::new("test");
        let expect = format_input(&input);
        assert_eq!(expect, "Test");
    }

    #[test]
    fn it_should_capitalize_string_with_hifens() {
        let input = Recipient::new("new-test");
        let expect = format_input(&input);
        assert_eq!(expect, "NewTest");
    }

}