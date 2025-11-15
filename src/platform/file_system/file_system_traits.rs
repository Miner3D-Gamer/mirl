// #[const_trait]
// /// A helper trait for file systems so the new function is uniform
// pub trait FileSystemNew {
//     /// Create a new file system access-er, files that are not defined in `required_files` are not guaranteed to exist
//     ///
//     /// # Errors
//     /// If the required files cannot be found, an error will return
//     fn new() -> Self
//     where
//         Self: Sized;
// }

#[const_trait]
/// A trait for a simple file system for possible portability
pub trait FileSystem {
    /// # Get the contents of a file
    ///
    /// # Errors
    /// If the file is not found or otherwise accessible an error is returned
    fn get_file_contents(
        &self,
        path: &str,
    ) -> Result<super::FileData, Box<dyn std::error::Error>>;
    /// Write the desired data into the specified file in byte format
    ///
    /// # Errors
    /// If the file cannot be written to, it errors ¯\_(ツ)_/¯
    fn write_to_file(&self, path: &str, contents: &[u8])
        -> std::io::Result<()>;
    /// Get all file paths in the requested folder
    fn get_files_in_folder(&self, path: &str) -> Vec<String>;
    /// Get all sub folder paths in the requested folder
    fn get_folders_in_folder(&self, path: &str) -> Vec<String>;
    /// Join 2 paths together
    fn join(&self, path1: &str, path2: &str) -> String;
    /// Checks if a file exists
    fn does_file_exist(&self, path: &str) -> bool;
    /// Debug function to see what folders the implementation searched in
    fn get_searched_folders(&self) -> Vec<String>;
}
