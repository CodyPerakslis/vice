use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Debug, Serialize, Deserialize)]
pub struct Vice {
    pub name: String,
}

pub fn read_file() -> Result<Vice> {
    let file = File::open("fake_data.json").unwrap();
    let reader = BufReader::new(file);
    let vice: Vice = serde_json::from_reader(reader).unwrap();

    Ok(vice)
}

pub fn write_file(vice: Vice) -> Result<()> {
    let file = File::create("fake_data.json").unwrap();
    serde_json::to_writer(file, &vice).unwrap();

    Ok(())
}

