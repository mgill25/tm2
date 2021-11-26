use crate::s;
use log::error;
use std::{fs::File, io::{Read, Write}};

pub fn write(filepath: &str, data: String) -> bool {
    std::fs::write(filepath, data).is_ok()
}

pub fn remove(filepath: &str) -> bool {
    std::fs::remove_file(filepath).is_ok()
}

pub fn exists(filepath: &str) -> bool {
    std::fs::metadata(filepath).is_ok()
}

/// Reads a file and returns the Stringified data or Error
pub fn read_from_file(filepath: &str) -> String {
    let file_result = File::open(filepath);
    match file_result {
        Ok(mut file) => {
            let mut file_contents = String::new();
            if file.read_to_string(&mut file_contents).is_ok() {
                file_contents
            } else {
                s("")
            }
        }
        Err(err) => {
            error!("Failed to open file {}: {}", filepath, err);
            s("")
        }
    }
}

/**
 * Substitute inside a file: change the string source to target.
 * Reads, replaces and then overwrites the file.
 */
pub fn sed(filepath: &str, source: &str, target: &str) -> Result<(), std::io::Error> {
    let mut src = File::open(&filepath)?;
    let mut file_content = String::new();
    src.read_to_string(&mut file_content)?;
    drop(src); // close the file early (why?)
    let new_content = file_content.replace(source, target);
    let mut dst = File::create(&filepath)?;
    dst.write_all(new_content.as_bytes())?;
    Ok(())
}