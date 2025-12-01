use std::io::Read;

use crate::platform::file_system::{
    file_data::DataType, file_system_traits::FileSystem, FileData,
};

/// Implementation of `FileSystem` for Native use only
/// This is supposed to make accessing files easier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFileSystem {
    exe_path: std::path::PathBuf,
    src_path: Option<std::path::PathBuf>,
}
impl NativeFileSystem {
    /// # Errors
    /// When the current exe does not have a parent path, or the current exe does not exist. Both of which should not be possible under normal circumstances
    pub fn new(//required_files: Vec<&'static str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let temp = std::env::current_exe()?;
        let exe_path =
            temp.parent().ok_or("No parent for current_exe")?.to_path_buf();

        let src_path;
        if let Some(build_folder) = exe_path.parent() {
            if let Some(src_parent) = build_folder.parent() {
                if let Some(x) = src_parent.parent() {
                    src_path = Some(x.join("src"));
                } else {
                    src_path = None;
                }
            } else {
                src_path = None;
            }
        } else {
            src_path = None;
        }

        let file_system = Self {
            exe_path,
            src_path,
        };

        // for i in required_files {
        //     assert!(
        //         file_system.does_file_exist(i),
        //         "Searched: {:?} but could not find {}",
        //         file_system.get_searched_folders(),
        //         i
        //     );
        // }

        Ok(file_system)
    }
}

impl FileSystem for NativeFileSystem {
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
            return Ok(FileData::from_bytes(buffer, DataType::Bytes));
        }

        if let Some(src_path) = &self.src_path {
            // Try src path
            let src_full_path = src_path.join(path);

            let mut file = std::fs::File::open(&src_full_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            return Ok(FileData::from_bytes(buffer, DataType::Bytes));
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Requested file could not be found",
        )))
    }

    fn write_to_file(
        &self,
        path: &str,
        contents: &[u8],
    ) -> std::io::Result<()> {
        std::fs::write(self.exe_path.join(path), contents)?;
        Ok(())
    }
    fn get_files_in_folder(&self, path: &str) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(self.exe_path.join(path)) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Some(file_name) = entry_path.file_name() {
                        files.push(file_name.to_string_lossy().to_string());
                    }
                }
            }
        } else if let Some(origin) = &self.src_path {
            if let Ok(entries) = std::fs::read_dir(origin.join(path)) {
                for entry in entries.filter_map(Result::ok) {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(file_name) = entry_path.file_name() {
                            files.push(file_name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        files
    }

    fn get_folders_in_folder(&self, path: &str) -> Vec<String> {
        let mut folders = Vec::new();

        if let Ok(entries) = std::fs::read_dir(self.exe_path.join(path)) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if let Some(folder_name) = entry_path.file_name() {
                        folders.push(folder_name.to_string_lossy().to_string());
                    }
                }
            }
        } else if let Ok(entries) = std::fs::read_dir(
            self.src_path.clone().unwrap_or_default().join(path),
        ) {
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if let Some(folder_name) = entry_path.file_name() {
                        folders.push(folder_name.to_string_lossy().to_string());
                    }
                }
            }
        }
        folders
    }
    fn join(&self, path1: &str, path2: &str) -> String {
        std::path::Path::new(path1).join(path2).to_string_lossy().to_string()
    }
    fn does_file_exist(&self, path: &str) -> bool {
        if std::fs::metadata(self.exe_path.join(path)).is_ok() {
            return true;
        }
        if let Some(src_path) = &self.src_path {
            return std::fs::metadata(src_path.join(path)).is_ok();
        }
        false
    }
    fn does_folder_exist(&self, path: &str) -> bool {
        if std::fs::exists(self.exe_path.join(path)).is_ok() {
            return true;
        }
        if let Some(src_path) = &self.src_path {
            return std::fs::exists(src_path.join(path)).is_ok();
        }
        false
    }
    fn get_searched_folders(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.exe_path.to_str().unwrap_or_default().to_string());
        if let Some(value) = &self.src_path {
            vec.push(value.to_str().unwrap_or_default().to_string());
        }
        vec
    }
}
