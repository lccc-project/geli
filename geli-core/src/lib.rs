use std::io::Read;

use geli_types::Pipeline;

pub fn read_pipeline(file: impl Read) -> Pipeline {
    serde_json::from_reader(file).unwrap()
}
