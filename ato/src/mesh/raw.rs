use std::path::PathBuf;
use md5::{Digest, Md5};
use rayon::vec;

#[derive(Debug)]
pub struct Raw {
    f: PathBuf,
    e: String
}

impl Raw {
    pub fn new(f: PathBuf, e: String) -> Self {
        Self { f, e } 
    }

    pub fn ext(&self) -> String {
        self.e.clone()
    }

    pub fn from(dir: &str, extension: &str, pattern: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .map(|x| x.path())
            .filter(|f| f.extension().unwrap().to_str().unwrap().contains(extension))
            .filter(|f| f.file_name().unwrap().to_str().unwrap().contains(pattern))
            .collect();

        let f = files.first().ok_or("Not STL file")?.to_path_buf();
        let e = extension.to_string();
        Ok(Self::new(f, e))
    }

    pub fn cp(&self, to: PathBuf) -> Result<(), std::io::Error> {
        if !to.exists() {
            std::fs::copy(&self.f, to)?;
        }
        Ok(())
    }

    pub fn hash(&self) -> String {
        let mut hasher = Md5::new();
        let buff = std::fs::read(&self.f).unwrap();
        hasher.update(&buff);
        let digest = hasher.finalize();
        format!("{:x}", digest)
    }

    pub fn hash_vec(&self) -> Vec<u8> {
        let mut hasher = Md5::new();
        let buff = std::fs::read(&self.f).unwrap();
        hasher.update(&buff);
        let digest = hasher.finalize();
        digest.to_vec()
    }
}
