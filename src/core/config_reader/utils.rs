use std::{env, io, path::Path};

use super::keys::KEY_UNAME;

pub(in crate::core::config_reader) fn config_file_check() -> Result<String, io::Error> {
    let base_path = "adel.conf";

    if !Path::new(base_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound, 
            format!("Unable to locate config file: {}", base_path)
        ));
    }

    Ok(base_path.to_string())
}

pub(in crate::core::config_reader) fn edit_path_uname(path: &str) -> Option<String> {
    if !path.contains(KEY_UNAME) {
        return Some(path.to_string());
    }

    let username = env::var("USER").or_else(|_| env::var("USERNAME"));
    if let Ok(username) = username {
        return Some(path.to_string().replace(KEY_UNAME, username.as_str()));
    }

    None
}
