use std::io::Write;
use std::fs::File;

pub fn write_to_file(mut output: &File, text_to_write: &str) {
  match write!(output, "{}\n\n", text_to_write) {
    Ok(_) => (),
    Err(e) => println!("Failed to write to file, with error: {}", e)
  }
}
