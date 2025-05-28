use std::{fs::{metadata, create_dir, remove_dir_all}, path::Path};
use std::time::{Duration, SystemTime};

use log::info;

pub(in crate::core::worker) fn create_home(file_path: &str) -> std::io::Result<()> {
    if !Path::new(file_path).exists() {
        create_dir(file_path)?;

        info!("Home directory created: {}", file_path);
    }

    Ok(())
}

pub(in crate::core::worker) fn delete_adel_home(adel_home_path: &str, days: u64) -> std::io::Result<()> {
    let path = Path::new(adel_home_path);
    if path.is_dir() {
        let meta = metadata(path)?;
        if let Ok(modified_time) = meta.modified() {
            let now = SystemTime::now();
            if let Ok(duration) = now.duration_since(modified_time) {
                if duration > Duration::from_secs(days * 24 * 60 * 60) {
                    remove_dir_all(path)?;
                    info!("Deleted directory: {}", adel_home_path);
                } 
            }
        }
    } 

    create_home(adel_home_path)?;

    Ok(())
}