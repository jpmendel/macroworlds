use crate::gui::files::FileHandle;
use crate::gui::highlighter::highlighter::Highlighter;
use eframe::egui::FontId;
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
    pub open_files: Vec<FileHandle>,
}

impl Editor {
    pub fn new(font: FontId) -> Editor {
        let open_files = vec![FileHandle::new(String::from("untitled.logo"))];
        Editor {
            font,
            should_highlight: true,
            highlighter: Highlighter::new(),
            current_file_index: Some(0),
            open_files,
        }
    }

    pub fn get_file_mut(&mut self, index: usize) -> Option<&mut FileHandle> {
        self.open_files.get_mut(index)
    }

    pub fn current_file(&self) -> Option<&FileHandle> {
        let Some(index) = self.current_file_index else {
            return None;
        };
        self.open_files.get(index)
    }

    pub fn current_file_mut(&mut self) -> Option<&mut FileHandle> {
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

    pub fn close_current_file(&mut self) {
        let Some(index) = self.current_file_index else {
            return;
        };
        self.close_file(index);
    }

    pub fn new_file(&mut self) {
        let file = FileHandle::new(String::from("untitled.logo"));
        self.open_files.push(file);
        self.current_file_index = Some(self.open_files.len() - 1);
    }

    pub fn open_file(&mut self) -> bool {
        let file_path = FileDialog::new()
            .add_filter("logo", &["txt", "logo"])
            .set_directory(".")
            .pick_file();
        let Some(file_path) = file_path else {
            return false;
        };
        let Ok(mut file) = File::open(file_path.clone()) else {
            return false;
        };
        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            println!("Failed to load file: {}", err);
            return false;
        }
        if let Some(file_name) = file_path.clone().file_name() {
            let file_name = file_name.to_string_lossy().to_string();
            let file = FileHandle::from_open(file_name, file_path, contents);
            self.open_files.push(file);
            self.current_file_index = Some(self.open_files.len() - 1);
        }
        true
    }

    pub fn save_current_file(&mut self) {
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
        let Some(file_path) = file_path else {
            return;
        };
        let Ok(mut file) = File::create(file_path.clone()) else {
            return;
        };
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
