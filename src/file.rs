use failure::Error;
use quicli::fs::{write_to_file, read_file};
use std::path::PathBuf;

pub fn read_cache_file(path: &PathBuf) -> Result<String, Error> {
    Ok(read_file(path)?)
}

pub fn write_cache_file(path: &PathBuf, ip: &str) -> Result<(), Error> {
    write_to_file(path, ip)?;
    Ok(())
}
