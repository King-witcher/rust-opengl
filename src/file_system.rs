use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::Read,
    path::Path,
};

use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("Failed to load base files")]
    IoError(std::io::Error),

    #[error("Invalid base files")]
    ZipError(zip::result::ZipError),

    #[error("Resource not found: \"{0}\"")]
    ResourceNotFound(String),
}

type FileSystemResult<T> = Result<T, FileSystemError>;

pub struct FileSystem {
    files: HashMap<String, Vec<u8>>,
}

impl FileSystem {
    pub fn get(&self, name: &str) -> FileSystemResult<&Vec<u8>> {
        let result = self.files.get(name);

        if let Some(data) = result {
            Ok(data)
        } else {
            Err(FileSystemError::ResourceNotFound(name.to_string()))
        }
    }
}

pub fn load_file_system() -> FileSystemResult<FileSystem> {
    let base_path = Path::new("base");
    let paths = fs::read_dir(base_path)?;
    let mut hash_map = HashMap::new();
    for entry in paths {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() && path.extension() == Some(OsStr::new("pk5")) {
            let files_loaded = load_pack(&mut hash_map, &path)?;
            println!(
                "Loaded {} files from {}",
                files_loaded,
                path.to_str().unwrap()
            );
        }
    }

    Ok(FileSystem { files: hash_map })
}

fn load_pack(hash_map: &mut HashMap<String, Vec<u8>>, path: &Path) -> FileSystemResult<usize> {
    let file = File::open(path)?;
    let mut count = 0;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let name = file.name().to_string();
        let mut bytes = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut bytes)?;
        if !hash_map.contains_key(&name) {
            println!("load {} ({} bytes)", name, bytes.len());
            hash_map.insert(name, bytes);
            count += 1;
        }
    }
    Ok(count)
}

impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        FileSystemError::IoError(err)
    }
}

impl From<zip::result::ZipError> for FileSystemError {
    fn from(err: zip::result::ZipError) -> Self {
        FileSystemError::ZipError(err)
    }
}
