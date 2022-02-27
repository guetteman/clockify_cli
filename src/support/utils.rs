use std::env::var;
use owo_colors::{OwoColorize, colors::Red};

pub fn print_error(message: String) {
    let internal_message = format!(" {} ", message);
    println!(
        "{}",
        internal_message.bg::<Red>(),
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

