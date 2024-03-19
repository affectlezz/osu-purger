use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // Let's user input path to songs folder
    println!("Input path to songs folder:");
    let mut user_defined_path = String::new();
    let input = io::stdin();
    input.read_line(&mut user_defined_path).unwrap().to_string();

    // Defines target extensions and path
    let target_extensions = vec!["jpg", "jpeg", "bmp", "png", "mp3", "wav", "ogg"];
    let dir_path = Path::new(user_defined_path.trim());

    // Crawls given directory and checks if it corresponds to a file
    for element in WalkDir::new(dir_path) {
        let element = element.unwrap();
        if element.file_type().is_file() {
            let path = element.path();
            // If statement avoids errors from Err types, converts extension to str
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                // Converts extension to lowercase
                let lowercase = extension.to_lowercase();
                // If it contains a target extension and isn't called "audio"
                if target_extensions.contains(&lowercase.as_str())
                    && !path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .contains("audio")
                {
                    // Prints file path and deletes
                    println!("Deleted file: {:?}", path);
                    fs::remove_file(path).unwrap();
                }
            }
        }
    }
}
