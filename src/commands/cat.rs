use std::fs;

pub fn process_cat(arguments: &Vec<String>) -> i32 {
    // for argument in arguments {
    //     print!("{} ,", argument);
    // }
    for filename in arguments {
        let content = fs::read_to_string(filename);
        match content {
            Ok(file_content) => {
                print!("{} ", file_content);
            }
            Err(_) => {
                eprintln!("Failed to read file");
                return -1;
            }
        }
    }
    0
}
