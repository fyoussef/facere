#[derive(Debug)]
pub struct Recipient {
    pub filename: String,
    pub directories: String,
    pub full_path: String,
}

impl Recipient {
    pub fn new(input: &str) -> Recipient {
        let mut split_path: Vec<&str> = input.split('/').collect();
        let file = split_path.pop().unwrap();
        let filename = file.to_string() + ".ts";
        let directories = Self::remove_file_from_dirs(input, file);

        Recipient {
            filename,
            directories: if !directories.is_empty() { Self::include_src_dir(directories) } else { "".to_string() },
            full_path: Self::include_src_dir(input.to_string() + ".ts"),
        }
    }

    fn remove_file_from_dirs(input: &str, file: &str) -> String {
        input.to_string().replace(file, "")
    }

    fn include_src_dir(input: String) -> String {
        format!("src/{}", input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Recipient;

    #[test]
    fn it_should_create_a_recipient_with_dir_and_file() {
        let recipient = Recipient::new("my/path");
        assert_eq!(recipient.filename, "path.ts");
        assert_eq!(recipient.full_path, "src/my/path.ts");
        assert_eq!(recipient.directories, "src/my/");
    }

    #[test]
    fn it_should_create_a_recipient_with_file() {
        let recipient = Recipient::new("path");
        assert_eq!(recipient.filename, "path.ts");
        assert_eq!(recipient.full_path, "src/path.ts");
        assert_eq!(recipient.directories, "");
    }
}