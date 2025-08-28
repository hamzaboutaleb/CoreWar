pub mod tokenizer;
use std::path::Path;

pub fn is_valid_file(filename: &str) -> bool {
    if !filename.ends_with(".s") {
        return false;
    }
    if !Path::new(filename).exists() {
        return false;
    }
    return true;
}
