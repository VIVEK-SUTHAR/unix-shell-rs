use std::env;
pub fn my_echo(args: &Vec<String>) -> i32 {
    for argument in args {
        if argument.starts_with("$") {
            let system_variable = &argument[1..];
            match env::var(system_variable) {
                Ok(value) => print!("{} ", value),
                Err(_) => {}
            }
        } else {
            print!("{} ", argument);
        }
    }
    println!();
    0
}
