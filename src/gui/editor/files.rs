use eframe::egui::TextBuffer;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileHandle {
    pub name: String,
    pub path: Option<PathBuf>,
    pub content: String,
    pub is_edited: bool,
}

impl FileHandle {
    pub fn new(name: String) -> Self {
        FileHandle {
            name,
            path: None,
            content: String::new(),
            is_edited: false,
        }
    }

    pub fn from_open(name: String, path: PathBuf, content: String) -> Self {
        FileHandle {
            name,
            path: Some(path),
            content,
            is_edited: false,
        }
    }
}

impl TextBuffer for FileHandle {
    fn is_mutable(&self) -> bool {
        self.content.is_mutable()
    }

    fn as_str(&self) -> &str {
        self.content.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.is_edited = true;
        self.content.insert_text(text, char_index)
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.is_edited = true;
        self.content.delete_char_range(char_range)
    }
}
