use std::{fs::File, io::Read};

use dev_cli::{devfile::validator::version::DevFileVersion, trace};
use tracing::{event, Level};

extern crate dev_cli;

pub fn main() {
    trace::init::init_tracing(Level::INFO, false);
    let mut dev_filefs = match File::open("./devfile.yaml") {
        Ok(file) => file,
        Err(err) => {
            event!(tracing::Level::ERROR, "Could not open file: {:?}", err);
            return;
        }
    };
    let mut contents = String::new();
    match dev_filefs.read_to_string(&mut contents) {
        Ok(content) => content,
        Err(err) => {
            event!(tracing::Level::ERROR, "Could not read file: {:?}", err);
            return;
        }
    };
    let version = DevFileVersion::validate(contents.clone());
    println!("{:?}", version);
}
