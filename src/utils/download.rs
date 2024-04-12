use reqwest::blocking::get;
use std::fs::write;
use std::path::PathBuf;

pub trait Download {
    fn silence_output(&self) -> bool;

    fn download(&self, save_location: &PathBuf) -> Result<(), String>;
}

pub struct HttpDownloader {
    pub url: String,
    pub silent: bool,
}

impl Download for HttpDownloader {
    fn silence_output(&self) -> bool {
        self.silent
    }

    fn download(&self, save_location: &PathBuf) -> Result<(), String> {
        let response = get(&self.url)
            .map_err(|_| "Request to ChEMBL README failed".to_string())?
            .error_for_status()
            .map_err(|_| "Request failed with error status code".to_string())?;
        let content = response
            .bytes()
            .map_err(|_| "Unable to read file".to_string())?;
        write(save_location, content).map_err(|_| "Unable to write file")?;
        Ok(())
    }
}
