use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
    time::{Duration, Instant},
};

use arboard::{Clipboard, SetExtLinux};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum YankError {
    #[error("No key provided")]
    NoKeyProvided,

    #[error("Could not find home directory")]
    HomeDirNotFound,

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to parse data file: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Clipboard error: {0}")]
    Clipboard(#[from] arboard::Error),

    #[error("No value found for '{0}'")]
    KeyNotFound(String),
}

pub type Result<T> = std::result::Result<T, YankError>;

pub struct Handler {
    file_path: PathBuf,
    data: Value,
}

impl Handler {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or(YankError::HomeDirNotFound)?;
        let file_path = home.join(".yank/data.json");

        if let Some(dir) = file_path.parent() {
            fs::create_dir_all(dir)?;
        }

        Ok(Self {
            file_path,
            data: Value::default(),
        })
    }

    pub fn load_data(&mut self) -> Result<()> {
        match fs::read_to_string(&self.file_path) {
            Ok(content) => {
                let data: Value = serde_json::from_str(&content)?;
                self.data = data;
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                self.data = Value::Object(serde_json::Map::new());
            }
            Err(e) => return Err(YankError::Io(e)),
        }

        Ok(())
    }
    pub fn save_data(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn list_keys(&self) -> Result<()> {
        let map = match self.data.as_object() {
            Some(map) if !map.is_empty() => map,
            _ => {
                println!("No keys stored.");
                return Ok(());
            }
        };

        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();

        for key in keys {
            println!("{key}");
        }

        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Result<String> {
        let value = self
            .data
            .get(key)
            .ok_or_else(|| YankError::KeyNotFound(key.to_string()))?;

        Ok(match value {
            Value::String(s) => s.clone(),
            _ => value.to_string(),
        })
    }

    pub fn set_value(&mut self, key: &str, value: String) -> Result<()> {
        if let Value::Object(map) = &mut self.data {
            map.insert(key.to_string(), Value::String(value));
        } else {
            let mut new_map = serde_json::Map::new();
            new_map.insert(key.to_string(), Value::String(value));
            self.data = Value::Object(new_map);
        }
        println!("Value set successfully!");
        self.save_data()?;
        Ok(())
    }

    pub fn delete_value(&mut self, key: &str) -> Result<()> {
        if let Value::Object(map) = &mut self.data {
            map.remove(key);
            println!("Value deleted successfully!");
        }
        self.save_data()?;
        Ok(())
    }

    pub fn yank_value(&self, key: &str) -> Result<()> {
        let value = self.get_value(key)?;
        println!("{value}");

        let mut clipboard = Clipboard::new()?;

        clipboard
            .set()
            .wait_until(Instant::now() + Duration::from_millis(100))
            .text(value)?;

        println!("Copied to clipboard!");
        Ok(())
    }
}
