use crate::gui::highlighter::highlighter::Highlighter;
use eframe::egui::FontId;
use rfd::FileDialog;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Editor {
    pub code: String,
    pub font: FontId,
    pub highlighter: Highlighter,
    pub current_file: Option<FileDescription>,
}

impl Editor {
    pub fn new(font: FontId) -> Editor {
        Editor {
            code: String::new(),
            font,
            highlighter: Highlighter::new(),
            current_file: None,
        }
    }

    pub fn new_file(&mut self) {
        self.code = String::new();
        self.current_file = None;
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
                    self.code = contents;
                    if let Some(file_name) = file_path.clone().file_name() {
                        let file_name = file_name.to_string_lossy().to_string();
                        self.current_file = Some(FileDescription {
                            name: file_name,
                            path: file_path,
                        });
                    } else {
                        self.current_file = None;
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn save_file(&mut self) {
        let file_path: Option<PathBuf>;
        if let Some(current_file) = self.current_file.clone() {
            file_path = Some(current_file.path);
        } else {
            file_path = FileDialog::new()
                .set_file_name("untitled.logo")
                .set_directory(".")
                .save_file();
        }
        if let Some(file_path) = file_path {
            if let Ok(mut file) = File::create(file_path.clone()) {
                let code = self.code.clone();
                let file_name = file_path
                    .clone()
                    .file_name()
                    .unwrap_or(&OsStr::new("unknown"))
                    .to_string_lossy()
                    .to_string();
                match file.write_all(code.as_bytes()) {
                    Ok(..) => {
                        self.current_file = Some(FileDescription {
                            name: file_name,
                            path: file_path,
                        });
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
    pub path: PathBuf,
}
