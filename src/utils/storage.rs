use std::env;
use std::fs;
use std::path::PathBuf;

fn check_xdg_data_dirs() -> Result<PathBuf, String> {
    let data_dirs: Vec<String> = match env::var("XDG_DATA_DIRS") {
        Ok(value) => value.split(':').map(String::from).collect(),
        Err(_) => return Err("Env var unset".to_string()),
    };
    for data_dir in data_dirs {
        let path = PathBuf::from(data_dir);
        if !path.exists() || path.is_dir() {
            return Ok(path);
        }
    }
    Err("No valid paths given".to_string())
}

pub fn get_wags_tails_dir() -> Result<PathBuf, String> {
    let path = match env::var("WAGS_TAILS_DIR") {
        Ok(value) => PathBuf::from(value),
        Err(_) => match env::var("XDG_DATA_HOME") {
            Ok(value) => PathBuf::from(value),
            Err(_) => match check_xdg_data_dirs() {
                Ok(value) => value,
                Err(_) => PathBuf::from("/users/jss009/.local/share/wags_tails"),
            },
        },
    };

    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    if path.exists() {
        Ok(path)
    } else {
        Err("The path exists but is not a directory.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_get_dir_with_wags_tails_dir_set() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        env::set_var("WAGS_TAILS_DIR", temp_path);

        // ensure no contaminating env vars
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("XDG_DATA_DIRS");

        let result = get_wags_tails_dir();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_path);

        env::remove_var("WAGS_TAILS_DIR");
    }
}
