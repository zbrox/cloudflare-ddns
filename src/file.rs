use failure::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

pub fn read_cache_file(path: &PathBuf) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s.clone())
}

pub fn write_cache_file(path: &PathBuf, ip: &str) -> Result<(), Error> {
    let mut file = File::create(&path)?;
    file.write_all(ip.as_bytes())?;

    Ok(())
}