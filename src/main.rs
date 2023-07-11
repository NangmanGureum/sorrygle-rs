use std::env;

struct SorrygleArgs {
    filename: String,
    options: Vec<String>,
}

impl SorrygleArgs {
    fn new(args: &Vec<String>) -> Result<SorrygleArgs, String> {
        let mut is_file_ready = false;
        let mut options: Vec<String> = Vec::new();
        let mut filename: String = String::new();
        for arg in args {
            if arg.starts_with("-") {
                options.push(arg.to_owned());
            } else if arg.ends_with(".srg") {
                if !is_file_ready {
                    is_file_ready = true;
                    filename = arg.to_owned();
                }
            }
        }
        if filename == "".to_owned() {
            return Err("no input files".to_string());
        }
        Ok(SorrygleArgs { filename, options })
    }
}

struct SorrygleOptions {
    play: bool,
    covert: bool,
    strict: bool,
    extend: bool,
}

fn main() {
    let mut main_arg: SorrygleArgs;
}
