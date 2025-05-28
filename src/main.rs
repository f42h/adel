mod core;
use core::worker::session::Adel;

use log::info;

fn main() {
    env_logger::init();

    info!("Start session");

    if let Some(adel) = Adel::new() {
        adel.session();
    } 
}
