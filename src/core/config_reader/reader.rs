use std::{fs::read_to_string, io};

use super::{
    configs::Configurations, 
    keys::{
        KEY_ADEL_DIRS, KEY_DELAY_HOUR, KEY_DELAY_MIN, KEY_DELAY_SEC, KEY_DELETE_AFTER, KEY_PATH_TEMP_DIR
    }, 
    utils::{config_file_check, edit_path_uname}
};

pub(in crate::core::config_reader) fn read_config() -> Result<Configurations, io::Error> {
    let config_file = config_file_check()?;
    let mut settings = Configurations::new();
    let content = read_to_string(config_file)?;
    let read_settings: Vec<String> = content.lines().map(String::from).collect();

    for line in read_settings {
        if line.starts_with("#") {
            continue;
        }

        if line.starts_with(KEY_PATH_TEMP_DIR) {
            let mut value = line.trim_start_matches(KEY_PATH_TEMP_DIR).to_string();
            value = edit_path_uname(value.as_str()).unwrap_or(value);

            settings.set_temp_dir(value);
        }

        if line.starts_with(KEY_ADEL_DIRS) {
            let value = line.trim_start_matches(KEY_ADEL_DIRS);
            let mut dirs: Vec<String> = value.split(',').map(String::from).collect();

            for dir in dirs.iter_mut() {
                *dir = edit_path_uname(dir).unwrap_or(dir.to_string());
            }

            settings.set_adel_dirs(dirs)?;
        }

        if line.starts_with(KEY_DELAY_HOUR) {
            let value = line.trim_start_matches(KEY_DELAY_HOUR).parse::<u64>()
                .map_err(|_| io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "Invalid delay hour value"
                ))?;

            settings.set_delay_hour(value);
        }

        if line.starts_with(KEY_DELAY_MIN) {
            let value = line.trim_start_matches(KEY_DELAY_MIN).parse::<u64>()
                .map_err(|_| io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "Invalid delay minute value"
                ))?;

            settings.set_delay_min(value);
        }

        if line.starts_with(KEY_DELAY_SEC) {
            let value = line.trim_start_matches(KEY_DELAY_SEC).parse::<u64>()
                .map_err(|_| io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "Invalid delay second value"
                ))?;

            settings.set_delay_sec(value);
        }

        if line.starts_with(KEY_DELETE_AFTER) {
            let value = line.trim_start_matches(KEY_DELETE_AFTER).parse::<u64>()
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData, 
                "Invalid day setting"
            ))?;

            settings.set_delete_n(value);
        }
    }

    Ok(settings)
}

pub(in crate::core) fn get_configurations() -> Result<Configurations, io::Error>{
    Ok(read_config()?)
}