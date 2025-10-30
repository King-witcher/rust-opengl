use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, ffi::OsStr, fs, io::Read, path::Path};
use zip::ZipArchive;

pub struct EngineArchive {
    base: &'static Path,
    archives: HashMap<String, RefCell<ZipArchive<std::fs::File>>>,
    file_archive_map: HashMap<String, String>,
}

impl EngineArchive {
    pub fn new(base: &'static str) -> Result<Self> {
        let path = Path::new(base);
        let paths = path.read_dir()?;
        let mut archives = HashMap::new();
        let mut file_archive_map = HashMap::new();

        for path in paths {
            let path = path?.path();
            let Some(extension) = path.extension() else {
                continue;
            };
            if !Self::validate_extension(extension) {
                continue;
            }
            let metadata = fs::metadata(&path)?;
            if !metadata.is_file() {
                continue;
            }

            let archive_path = path.display().to_string();
            let file = fs::File::open(&archive_path)?;
            let archive = ZipArchive::new(file)?;
            for name in archive.file_names() {
                file_archive_map.insert(name.to_string(), archive_path.clone());
            }
            archives.insert(archive_path, RefCell::new(archive));
        }

        Ok(Self {
            base: path,
            archives,
            file_archive_map,
        })
    }

    pub fn load(&self, path: &str) -> Result<Vec<u8>> {
        if let Some(archive) = self.file_archive_map.get(path) {
            let mut archive = self.archives.get(archive).unwrap().borrow_mut();
            let mut file = archive.by_name(path)?;
            let mut bytes = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut bytes)?;
            Ok(bytes)
        } else {
            let path_buf = self.base.join(path);
            let metadata = fs::metadata(&path_buf)?;
            if !metadata.is_file() {
                anyhow::bail!("File cannot be a directory: {}", path_buf.display());
            }
            Ok(fs::read(path_buf)?)
        }
    }

    fn validate_extension(extension: &OsStr) -> bool {
        extension == OsStr::new("pak")
            || extension == OsStr::new("pk2")
            || extension == OsStr::new("pk3")
            || extension == OsStr::new("pk4")
    }
}
