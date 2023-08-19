use dirs;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Cfg {
    pub data_path: String,
    pub config_path: String,
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    PathError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(p) => write!(f, "Could not parse path to `{}` folder", p),
            Error::PathError(p) => write!(f, "Could not find `{}` folder", p),
            Error::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl Cfg {
    pub fn new() -> Result<Self, Error> {
        let data_path = dirs::data_dir()
            .ok_or_else(|| Error::PathError("data".to_string()))?
            .join("nsynk")
            .to_str()
            .ok_or(Error::PathError("data".to_string()))?
            .to_string();
        let config_path = dirs::config_dir()
            .ok_or_else(|| Error::PathError("config".to_string()))?
            .join("nsynk")
            .to_str()
            .ok_or(Error::PathError("config".to_string()))?
            .to_string();

        Ok(Cfg {
            data_path,
            config_path,
        })
    }

    pub fn initialize(&self) -> Result<(), Error> {
        let paths: [&PathBuf; 2] = [
            &Path::new(&self.data_path).to_path_buf(),
            &Path::new(&self.config_path).to_path_buf(),
        ];
        create_dirs(&paths)?;
        create_files(&paths)?;
        Ok(())
    }
}

fn create_dirs(paths: &[&PathBuf; 2]) -> Result<(), Error> {
    let results: Result<Vec<_>, _> = paths
        .iter()
        .map(|path| {
            if !path.exists() {
                fs::create_dir(path).map_err(|e| Error::IoError(e))?;
            }
            Ok(())
        })
        .collect();
    if let Err(e) = results {
        return Err(e);
    }
    Ok(())
}

fn create_files(paths: &[&PathBuf; 2]) -> Result<(), Error> {
    let db = paths[0].join("nsynk.db");
    let config = paths[1].join("config.yml");
    if !db.exists() {
        fs::File::create(db).map_err(|e| Error::IoError(e))?;
    }
    if !config.exists() {
        fs::File::create(config).map_err(|e| Error::IoError(e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_init_dirs() {
        let cfg = Cfg::new();
        let cfg = cfg.unwrap_or_else(|e| panic!("{e}"));
        cfg.initialize().unwrap_or_else(|e| panic!("{e}"));
    }
}
