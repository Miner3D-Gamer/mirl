
use crossterm::{cursor, terminal::Clear, terminal::ClearType, ExecutableCommand};
use std::io::stdout;
pub fn clear_console() {
    stdout()
        .execute(Clear(ClearType::All))
        .unwrap()
        .execute(cursor::MoveTo(0, 0))
        .unwrap();
}

pub fn print(msg: &str) {
    println!("{}", msg);
}

pub fn input(msg: &str) -> String {
    let mut input = String::new();
    print(msg);
    std::io::stdin().read_line(&mut input).unwrap();
    input.truncate(input.len() - 1);
    return input;
}