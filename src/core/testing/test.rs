mod test {
    use std::process::exit;

    use crate::core::config_reader::reader::{get_configurations};

    #[test] 
    fn config_test() {
        let conf = match get_configurations() {
            Ok(configs) => configs,
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(-1);
            } 
        };

        dbg!(conf);
    }
}