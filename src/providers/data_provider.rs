use crate::providers::chembl::ChemblData;
use crate::providers::drugsatfda::DrugsAtFDA;
use crate::providers::oncotree::OncoTreeData;
use crate::utils::storage;
use crate::utils::versioning;
use regex::Regex;
use std::path::PathBuf;
use std::fs;

pub trait DataProvider {
    fn src_name(&self) -> &str;
    fn file_type(&self) -> &str;

    fn file_name(&self) -> &str {
        self.src_name()
    }

    fn get_versioner(&self) -> Box<dyn versioning::DataVersion> {
        Box::new(versioning::WildcardVersion {})
    }

    fn get_latest_version(&self) -> Result<String, String>;

    fn download(&self, version: &str, save_location: &PathBuf) -> Result<(), String>;

    fn get_data_dir(&self) -> Result<PathBuf, String> {
        match storage::get_wags_tails_dir() {
            Ok(value) => Ok(value.join(self.src_name())),
            Err(value) => Err(value),
        }
    }

    fn file_pattern(&self) -> Regex {
        let pattern = format!(
            "{}_({}).{}",
            self.src_name(),
            self.get_versioner().pattern(),
            self.file_type()
        );
        Regex::new(&pattern).unwrap()
    }

    fn get_latest_data(&self, from_local: &bool) -> Result<PathBuf, String> {
        if *from_local {
            // get matching
            let entries = fs::read_dir(self.get_data_dir().unwrap()).unwrap();
            let filtered_entries = entries.filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.file_name().and_then(|name| name.to_str()).map_or(false, |name| self.file_pattern().is_match(name)) {
                        Some(path)
                    } else {
                        None
                    }
                })
            }).collect();
            // for entry in fs::read_dir(self.get_data_dir().unwrap()).unwrap() {
            //     let entry = entry.unwrap();
            //     let file_name = entry.file_name();
            //     let version = self.parse_version(file_name.to_str().unwrap());
            //     println!("{}", version.unwrap());
            // }

            // sort
            // return highest
        }
        let latest_version = self.get_latest_version()?;
        let latest_file_name = format!(
            "{}_{}.{}",
            self.src_name(),
            latest_version,
            self.file_type()
        );
        let latest_file_path = match self.get_data_dir() {
            Ok(value) => value.join(latest_file_name),
            Err(value) => return Err(value),
        };
        match latest_file_path.try_exists() {
            Ok(true) => Ok(latest_file_path),
            Ok(false) => {
                self.download(&latest_version, &latest_file_path)?;
                Ok(latest_file_path)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn parse_version(&self, file_name: &str) -> Result<String, String> {
        let caps = &self.file_pattern().captures(file_name).unwrap();
        let version_str = caps.get(1).unwrap().as_str();
        Ok(String::from(version_str))
    }
}

pub fn get_provider(source_name: &str) -> Result<Box<dyn DataProvider>, String> {
    let lowercase = &source_name.to_lowercase();

    match lowercase.as_str() {
        "chembl" => Ok(Box::new(ChemblData {})),
        "oncotree" => Ok(Box::new(OncoTreeData {})),
        "drugsatfda" => Ok(Box::new(DrugsAtFDA {})),
        _ => Err("error".to_string()),
    }
}
