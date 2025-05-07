pub struct NativeFileSystem {}
impl NativeFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}
use std::io::Read;

use super::{file_data::FileData, FileSystem};

impl FileSystem for NativeFileSystem {
    fn get_file_contents(
        &self,
        path: &str,
    ) -> Result<FileData, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(FileData::from_bytes(buffer))
    }

    fn write_to_file(&self, path: &str, contents: &str) {
        std::fs::write(path, contents).expect("Failed to write file");
    }
    fn get_files_in_folder(&self, path: &str) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Some(file_name) = entry_path.file_name() {
                        files.push(file_name.to_string_lossy().to_string());
                    }
                }
            }
        }
        return files;
    }

    fn get_folders_in_folder(&self, path: &str) -> Vec<String> {
        let mut folders = Vec::new();
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if let Some(folder_name) = entry_path.file_name() {
                        folders.push(folder_name.to_string_lossy().to_string());
                    }
                }
            }
        }
        return folders;
    }
    fn join(&self, path1: &str, path2: &str) -> String {
        return format!("{}/{}", path1, path2);
    }
    fn does_file_exist(&self, path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }
}
