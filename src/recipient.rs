
#[derive(Debug)]
pub struct Recipient {
    pub filename: String,
    pub path: String,
    pub full_path: String,
}

impl Recipient {
    pub fn new(input: &str) -> Recipient {
        let mut split_path: Vec<&str> = input.split('/').collect();
        let file = split_path.pop().unwrap();
        let filename = file.to_string() + ".ts";
        let path = input.to_string().replace(file, "");
    
        Recipient {
            filename,
            path,
            full_path: input.to_string() + ".ts",
        }
    }
}