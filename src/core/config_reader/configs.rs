use std::io;

#[derive(Debug)]
pub(in crate::core) struct Configurations {
    pub(in crate::core) path_temp_dir: String, 
    pub(in crate::core) adel_dirs: Vec<String>,
    pub(in crate::core) delay_hour: u64,
    pub(in crate::core) delay_min: u64,
    pub(in crate::core) delay_sec: u64,
    pub(in crate::core) delete_home_n: u64
}

impl Configurations{
    pub(in crate::core)  fn new() -> Self {
        Self { 
            path_temp_dir: String::new(), 
            adel_dirs: Vec::new(), 
            delay_hour: 0, 
            delay_min: 0, 
            delay_sec: 0,
            delete_home_n: 7
        }
    }

    pub(in crate::core)  fn set_temp_dir(&mut self, new_path_temp_dir: String) {
        self.path_temp_dir = new_path_temp_dir;
    }

    pub(in crate::core)  fn set_adel_dirs(&mut self, new_adel_dirs: Vec<String>) -> Result<(), io::Error> {
        if new_adel_dirs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unable to collect adel paths (are they separated by comma?)"
            ));
        }

        self.adel_dirs = new_adel_dirs;

        Ok(())
    }

    pub(in crate::core)  fn set_delay_hour(&mut self, new_delay_hour: u64) {
        self.delay_hour = new_delay_hour;
    }

    pub(in crate::core)  fn set_delay_min(&mut self, new_delay_min: u64) {
        self.delay_min = new_delay_min;
    }

    pub(in crate::core)  fn set_delay_sec(&mut self, new_delay_sec: u64) {
        self.delay_sec = new_delay_sec;
    }

    pub(in crate::core) fn set_delete_n(&mut self, new_delete_n: u64) {
        self.delete_home_n = new_delete_n;
    } 
}