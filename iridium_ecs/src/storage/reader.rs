/// Manages the process of reading data from storage.
pub struct StorageReader {
    /// The file that's been read.
    pub src: String,
}

impl StorageReader {
    /// Create a new reader.
    /// 
    /// Reads the `src_path` file.
    pub fn new(src_path: String) -> Option<Self> {
        let src = std::fs::read_to_string(src_path).ok()?;

        Some(StorageReader { src })
    }
}
