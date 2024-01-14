use std::{
    fs::File,
    io,
    io::prelude::*,
    io::BufReader,
};

const RESOURCES_PATH: &str = "./resources";

pub fn read_file(resource_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(format!("{RESOURCES_PATH}/{resource_path}"))?;
    Ok(BufReader::new(file))
}

pub fn read_file_as_string(resource_path: &str) -> io::Result<String> {
    let mut buf_reader = read_file(resource_path)?;
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn write_file(resource_path: &str, content: &String) -> io::Result<()> {
    File::create(format!("{RESOURCES_PATH}/{resource_path}"))?.write_all(content.as_bytes())
}