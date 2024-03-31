use std::error::Error;
use std::fs::{self, DirEntry};

pub trait KeyName {
    fn to_key_code(&self) -> Result<u8, Box<dyn Error>>;
}

pub trait KeyCode {
    fn to_key_name(&self) -> Result<String, Box<dyn Error>>;
}

impl KeyName for str {
    fn to_key_code(&self) -> Result<u8, Box<dyn Error>> {
        let ascii = match self {
            "space" => 32,
            "enter" => 10,
            "left" => 37,
            "up" => 38,
            "right" => 39,
            "down" => 40,
            chr if chr.len() == 1 => chr.chars().next().unwrap().to_ascii_lowercase() as u8,
            _ => return Err(Box::from("key is not an ascii character")),
        };
        Ok(ascii)
    }
}

impl KeyCode for u8 {
    fn to_key_name(&self) -> Result<String, Box<dyn Error>> {
        let key = match self {
            32 => String::from("space"),
            10 => String::from("enter"),
            37 => String::from("left"),
            38 => String::from("up"),
            39 => String::from("right"),
            40 => String::from("down"),
            num if num.is_ascii() => String::from(*num as char),
            _ => return Err(Box::from("number does not represent an ascii key")),
        };
        Ok(key)
    }
}

pub fn query_files(
    base_path: &String,
    query: fn(&DirEntry) -> bool,
) -> Result<String, Box<dyn Error>> {
    let mut files: Vec<String> = vec![];
    let entries = fs::read_dir(base_path)?;
    for entry in entries {
        let Ok(entry) = entry else { continue };
        if (query)(&entry) {
            let Ok(file_name) = entry.file_name().into_string() else {
                continue;
            };
            files.push(file_name);
        }
    }
    Ok(files.join(" "))
}
