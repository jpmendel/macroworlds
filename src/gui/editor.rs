use crate::gui::highlighter::highlighter::Highlighter;
use eframe::egui::{FontId, TextBuffer};
use rfd::FileDialog;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Editor {
    pub font: FontId,
    pub should_highlight: bool,
    pub highlighter: Highlighter,
    pub current_file_index: Option<usize>,
    pub open_files: Vec<FileDescription>,
}

impl Editor {
    pub fn new(font: FontId) -> Editor {
        let open_files = vec![FileDescription::new(String::from("untitled.logo"))];
        Editor {
            font,
            should_highlight: true,
            highlighter: Highlighter::new(),
            current_file_index: Some(0),
            open_files,
        }
    }

    pub fn get_file_mut(&mut self, index: usize) -> Option<&mut FileDescription> {
        self.open_files.get_mut(index)
    }

    pub fn current_file(&self) -> Option<&FileDescription> {
        let Some(index) = self.current_file_index else {
            return None;
        };
        self.open_files.get(index)
    }

    pub fn current_file_mut(&mut self) -> Option<&mut FileDescription> {
        let Some(index) = self.current_file_index else {
            return None;
        };
        self.open_files.get_mut(index)
    }

    pub fn current_code(&self) -> &str {
        let Some(file) = self.current_file() else {
            return "";
        };
        &file.content
    }

    pub fn select_file(&mut self, index: usize) {
        self.current_file_index = Some(index);
    }

    pub fn close_file(&mut self, index: usize) {
        self.open_files.remove(index);
        if self.open_files.is_empty() {
            self.current_file_index = None;
            return;
        }
        if let Some(current_index) = self.current_file_index {
            if current_index > self.open_files.len() - 1 {
                self.current_file_index = Some(self.open_files.len() - 1);
            }
        }
    }

    pub fn new_file(&mut self) {
        let file = FileDescription::new(String::from("untitled.logo"));
        self.open_files.push(file);
        self.current_file_index = Some(self.open_files.len() - 1);
    }

    pub fn open_file(&mut self) -> bool {
        let file_path = FileDialog::new()
            .add_filter("logo", &["txt", "logo"])
            .set_directory(".")
            .pick_file();
        if let Some(file_path) = file_path {
            if let Ok(mut file) = File::open(file_path.clone()) {
                let mut contents = String::new();
                if let Err(err) = file.read_to_string(&mut contents) {
                    println!("Failed to load file: {}", err);
                } else {
                    if let Some(file_name) = file_path.clone().file_name() {
                        let file_name = file_name.to_string_lossy().to_string();
                        let file = FileDescription::from_open(file_name, file_path, contents);
                        self.open_files.push(file);
                        self.current_file_index = Some(self.open_files.len() - 1);
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn save_file(&mut self) {
        let Some(current_file) = self.current_file() else {
            return;
        };
        let file_path: Option<PathBuf>;
        if let Some(path) = current_file.path.clone() {
            file_path = Some(path);
        } else {
            file_path = FileDialog::new()
                .set_file_name(current_file.name.as_str())
                .set_directory(".")
                .save_file();
        }
        if let Some(file_path) = file_path {
            if let Ok(mut file) = File::create(file_path.clone()) {
                let file_name = file_path
                    .clone()
                    .file_name()
                    .unwrap_or(&OsStr::new("unknown"))
                    .to_string_lossy()
                    .to_string();
                match file.write_all(current_file.content.as_bytes()) {
                    Ok(..) => {
                        let Some(current_file) = self.current_file_mut() else {
                            return;
                        };
                        current_file.name = file_name;
                        current_file.path = Some(file_path);
                        current_file.is_edited = false;
                    }
                    Err(err) => println!("Failed to save file: {}", err),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileDescription {
    pub name: String,
    pub path: Option<PathBuf>,
    pub content: String,
    pub is_edited: bool,
}

impl FileDescription {
    pub fn new(name: String) -> Self {
        FileDescription {
            name,
            path: None,
            content: String::new(),
            is_edited: false,
        }
    }

    pub fn from_open(name: String, path: PathBuf, content: String) -> Self {
        FileDescription {
            name,
            path: Some(path),
            content,
            is_edited: false,
        }
    }
}

impl TextBuffer for FileDescription {
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
