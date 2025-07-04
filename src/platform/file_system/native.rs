use std::io::Read;

use super::super::{file_data::FileData, FileSystem};
pub struct NativeFileSystem {
    exe_path: std::path::PathBuf,
    src_path: std::path::PathBuf,
}

impl FileSystem for NativeFileSystem {
    fn new(required_files: Vec<String>) -> Self {
        let temp = std::env::current_exe().unwrap();
        let exe_path = temp.parent().unwrap();

        let file_system = Self {
            exe_path: exe_path.to_path_buf(),
            src_path: exe_path.parent().unwrap().parent().unwrap().join("src"),
        };
        for i in required_files {
            if !file_system.does_file_exist(&i) {
                panic!("File {} could not be found", i);
            }
        }
        file_system
    }
    fn get_file_contents(
        &self,
        path: &str,
    ) -> Result<FileData, Box<dyn std::error::Error>> {
        // Try exe path first
        let exe_full_path = self.exe_path.join(path);
        let exe_check = std::fs::File::open(&exe_full_path);

        if let Ok(mut file) = exe_check {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            return Ok(FileData::from_bytes(buffer));
        }

        // Try src path
        let src_full_path = self.src_path.join(path);

        let mut file = std::fs::File::open(&src_full_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(FileData::from_bytes(buffer))
    }

    fn write_to_file(&self, path: &str, contents: &[u8]) {
        std::fs::write(&self.exe_path.join(path), contents)
            .expect("Failed to write file");
    }
    fn get_files_in_folder(&self, path: &str) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.exe_path.join(path)) {
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
        if let Ok(entries) = std::fs::read_dir(&self.exe_path.join(path)) {
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
        return std::env::join_paths([path1, path2])
            .unwrap()
            .to_string_lossy()
            .to_string();
    }
    fn does_file_exist(&self, path: &str) -> bool {
        std::fs::metadata(&self.exe_path.join(path)).is_ok()
            || std::fs::metadata(&self.src_path.join(path)).is_ok()
    }
}
