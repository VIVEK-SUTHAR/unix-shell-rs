use std::env;
use std::fs;
pub fn print_contents_of_current_directory() -> i32 {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Failed to get the current working directory");
            return -1;
        }
    };

    let paths = match fs::read_dir(&current_dir) {
        Ok(paths) => paths,
        Err(_) => {
            eprintln!("Failed to read the current directory");
            return -1;
        }
    };

    for path_result in paths {
        match path_result {
            Ok(entry) => {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if path.is_file() {
                        print!("{} ", file_name.to_string_lossy());
                    } else {
                        let purple = "\x1b[34m";
                        let reset = "\x1b[0m";
                        print!("{}{}/{} ", purple, file_name.to_string_lossy(), reset);
                    }
                } else {
                    eprintln!("Failed to extract file name");
                }
            }
            Err(_) => {
                eprintln!("Failed to read an entry in the directory");
            }
        }
    }
    println!();
    0
}
