use crate::providers::data_provider::DataProvider;
use crate::utils::download::{Download, HttpDownloader};
use regex::Regex;
use reqwest::blocking::get;
use std::path::PathBuf;

pub struct ChemblData {}

impl DataProvider for ChemblData {
    fn src_name(&self) -> &str {
        "chembl"
    }

    fn file_type(&self) -> &str {
        "db"
    }

    fn get_latest_version(&self) -> Result<String, String> {
        let readme_url = "https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/README";
        let text = get(readme_url)
            .map_err(|_| "Request to ChEMBL README failed".to_string())?
            .error_for_status()
            .map_err(|_| "Request failed with error status code".to_string())?
            .text()
            .map_err(|_| "Unable to read ChEMBL README".to_string())?;
        let pattern = Regex::new(r"Release:\s+chembl_(\d{2})").unwrap();

        if let Some(caps) = pattern.captures(&text) {
            if let Some(matched) = caps.get(1) {
                Ok(matched.as_str().to_string())
            } else {
                Err("Failed to parse release number from line".to_string())
            }
        } else {
            Err("Failed to parse release number from README".to_string())
        }
    }

    fn download(&self, version: &str, save_location: &PathBuf) -> Result<(), String> {
        let url = format!(
            "https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/chembl_{}_sqlite.tar.gz",
            version
        );
        let downloader = HttpDownloader { url, silent: false };
        downloader.download(save_location)?;
        Ok(())
    }
}
