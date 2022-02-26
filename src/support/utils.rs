use termion::color;

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
