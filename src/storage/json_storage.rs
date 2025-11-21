use std::fs::{self, File};
use std::path::Path;
use serde_json;
use std::io::Write;

use crate::models::Task;

pub struct JsonManager {
    file_path: String,
}

impl JsonManager {

    // Set new file path
    pub fn new(file_path: String) -> Self {
        // Extraer el directorio del path completo
        if let Some(parent) = Path::new(&file_path).parent() {
            fs::create_dir_all(parent).ok();
        }
        JsonManager { file_path }
    }

    // Write tasks
    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(tasks)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    // Get existing tasks
    pub fn load_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {

        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let data = fs::read_to_string(&self.file_path)?;

        let tasks: Vec<Task> = serde_json::from_str(&data)?;

        Ok(tasks)

    }
}