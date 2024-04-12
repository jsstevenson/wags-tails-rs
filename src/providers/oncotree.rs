use crate::providers::data_provider::DataProvider;
use crate::utils::download::{Download, HttpDownloader};
use reqwest::blocking::get;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct VersionApiResponse {
    release_date: String,
}

pub struct OncoTreeData {}

impl DataProvider for OncoTreeData {
    fn src_name(&self) -> &str {
        "oncotree"
    }

    fn file_type(&self) -> &str {
        "json"
    }

    fn get_latest_version(&self) -> Result<String, String> {
        let response = get("http://oncotree.info/api/versions")
            .map_err(|_| "Request to OncoTree versions endpoint failed".to_string())?
            .error_for_status()
            .map_err(|_| "Request failed with error status code".to_string())?;
        if let Ok(data) = response.json::<Vec<VersionApiResponse>>() {
            if let Some(version_data) = data.first() {
                Ok(version_data.release_date.clone())
            } else {
                Err("Empty json response".to_string())
            }
        } else {
            Err("invalid json response".to_string())
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
