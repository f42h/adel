use std::{fs, io, path::Path, thread, time::Duration};

use crate::{core::{
    config_reader::{
        configs::Configurations, 
        reader::get_configurations
    },
    worker::utils::{check_create_home, delete_adel_home}
}, scandelay};
use log::{debug, error, info};

pub(crate) struct Adel {
    configs: Configurations
}

impl Adel {
    pub(crate) fn new() -> Option<Self> {
        info!("Try to parse configuration file");

        let read_config_file = match get_configurations() {
            Ok(content) => content,
            Err(err) => {
                error!("Error: {}", err);
                return None;
            }
        };

        Some(Self { 
            configs: read_config_file
        })
    }

    fn scan_dir(&self, dir: &str) -> Result<Vec<String>, io::Error> {
        info!("Scanning {} for jobs", dir);

        let mut moved: Vec<String> = Vec::new(); 
        let paths = fs::read_dir(dir).unwrap();

        for path in paths {
            let current = path.unwrap().path().display().to_string();

            if current.ends_with(".adel") {
                let mut temp_path = self.configs.path_temp_dir.to_string();

                if !temp_path.ends_with('/') {
                    temp_path.push('/');
                }

                let filename = Path::new(&current).file_name().ok_or_else(|| 
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("File not found: {}", current)
                    ))?;
                    
                let destination = format!("{}{}", temp_path, filename.to_string_lossy());

                fs::rename(&current, &destination)?;
                moved.push(current.to_string());
            }
        }

        Ok(moved)
    }

    pub(crate) fn session(&self) {
        debug!("Loaded settings:\n{:#?}", &self.configs);

        let home_dir = &self.configs.path_temp_dir;

        if let Err(err) = check_create_home(home_dir) {
            error!("Error: {}", err);
            return;
        } 

        let delay = scandelay!(self.configs.delay_hour, self.configs.delay_min, self.configs.delay_sec);

        info!("Set query delay to: {}", delay);
        info!("Set delete after n days where n={}", self.configs.delete_home_n);

        loop {
            if let Err(err) = delete_adel_home(&self.configs.path_temp_dir, self.configs.delete_home_n) {
                error!("Error: {}", err);
                return;
            }

            for dir in &self.configs.adel_dirs {
                if let Ok(moved) = self.scan_dir(dir) {
                    debug!("Cleared:\n{:#?}", moved);
                }
            } 

            thread::sleep(Duration::from_secs(delay));
        }
    }
}