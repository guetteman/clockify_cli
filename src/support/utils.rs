use termion::color;
use std::env::var;

pub fn print_error(message: String) {
    println!(
        "{} {} ",
        color::Bg(color::Red),
        message
    );
}

pub fn print_error_and_exit(message: String) -> ! {
    print_error(message);
    std::process::exit(1);
}

pub fn get_env_var(name: String) -> String {
    match var(&name) {
        Ok(result) => result,
        Err(_) => {
            print_error_and_exit(format!("\"{}\" not found", name))
        },
    }
}
