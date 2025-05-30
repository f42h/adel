use std::{fs::{create_dir, metadata, remove_dir_all}, io, path::Path};
use std::time::{Duration, SystemTime};

use log::info;

use crate::deletedelay;

pub(in crate::core::worker) fn check_create_home(file_path: &str) -> std::io::Result<()> {
    if !Path::new(file_path).exists() {
        create_dir(file_path)?;

        info!("Home directory created: {}", file_path);
    }

    Ok(())
}

pub(in crate::core::worker) fn delete_adel_home(adel_home_path: &str, days: u64) -> Result<(), io::Error> {
    let path = Path::new(adel_home_path);
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory, 
            format!("Given path needs to point to a directory: {}", adel_home_path)
        ));
    }

    let meta = metadata(path)?;
    let mod_time = meta.modified()?;
    let now = SystemTime::now();

    if let Ok(duration) = now.duration_since(mod_time) {
        if duration > Duration::from_secs(deletedelay!(days)) {
            remove_dir_all(path)?;

            info!("Deleted directory: {}", adel_home_path);
        } 
    }

    check_create_home(adel_home_path)?;

    Ok(())
}