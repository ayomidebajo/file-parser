use std::fmt::Display;

#[derive(Debug)]
pub enum FileType {
    TXT,
    PDF,
    PNG,
    JPEG,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FileType {
    pub fn parse(&self) -> String {
        match self {
            FileType::TXT => String::from("txt"),
            FileType::PDF => String::from("pdf"),
            FileType::PNG => String::from("png"),
            FileType::JPEG => String::from("jpeg"),
        }
    }
}
