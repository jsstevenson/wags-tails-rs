use crate::providers::data_provider::DataProvider;
use crate::utils::versioning::{DataVersion, IsoDateVersion};

pub struct DrugsAtFDA {}

impl DataProvider for DrugsAtFDA {
    fn src_name(&self) -> &str {
        "drugsatfda"
    }

    fn file_type(&self) -> &str {
        "json"
    }

    fn get_versioner(&self) -> Box<dyn DataVersion> {
        Box::new(IsoDateVersion {})
    }

    fn get_latest_version(&self) -> Result<String, String> {
        return Ok("ok".to_string());
    }

    fn download(&self, version: &str, save_location: &std::path::PathBuf) -> Result<(), String> {
        return Ok(());
    }
}
