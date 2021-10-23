use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use anyhow::Result;

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn write_file(path: &Path, ip: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(ip.as_bytes())?;
    Ok(())
}
