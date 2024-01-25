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

#[cfg(test)]
mod tests {
    use super::Recipient;

    #[test]
    fn it_should_create_a_recipient_with_dir_and_file() {
        let recipient = Recipient::new("my/path");
        assert_eq!(recipient.filename, "path.ts");
        assert_eq!(recipient.full_path, "my/path.ts");
        assert_eq!(recipient.path, "my/");
    }

    #[test]
    fn it_should_create_a_recipient_with_file() {
        let recipient = Recipient::new("path");
        assert_eq!(recipient.filename, "path.ts");
        assert_eq!(recipient.full_path, "path.ts");
        assert_eq!(recipient.path, "");
    }
}