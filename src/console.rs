use crate::graphics::Pixel;
use crossterm::{
    cursor, terminal::Clear, terminal::ClearType, ExecutableCommand,
};
use std::io::{stdout, BufRead, Write};

/// Clears the currently visible console
pub fn clear_console() {
    stdout()
        .execute(Clear(ClearType::All))
        .unwrap()
        .execute(cursor::MoveTo(0, 0))
        .unwrap();
}
/// Color the given text (requires the console to support the full color range)
pub fn color_text(msg: &str, r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, msg);
}
/// Color the background of the given text (requires the console to support the full color range)
pub fn color_background(msg: &str, r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, msg);
}
/// Color the text and color of the given string (requires the console to support the full color range)
pub fn color(
    msg: &str,
    r1: u8,
    g1: u8,
    b1: u8,
    r2: u8,
    g2: u8,
    b2: u8,
) -> String {
    return format!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
        r1, g1, b1, r2, g2, b2, msg
    );
}
/// Return the 'clear' all effects marker
pub fn reset_color() -> String {
    return "\x1b[0m".to_string();
}
/// Clear X lines
pub fn clear_lines(n: usize) {
    let mut stdout = std::io::stdout();

    for _ in 0..n {
        // Move cursor up one line
        write!(stdout, "\x1B[1A").unwrap();
        // Clear the entire line
        write!(stdout, "\x1B[2K").unwrap();
    }

    // Ensure the commands are flushed to the terminal
    stdout.flush().unwrap();
}
/// A python like input function
pub fn input(msg: &str) -> String {
    let mut input = String::new();
    println!("{msg}");
    std::io::stdin().read_line(&mut input).unwrap();
    input.truncate(input.len() - 1);
    return input;
}
/// Get the (full) content of the console
pub fn get_console_content(max_lines: usize) -> Vec<String> {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let mut recent_lines = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            recent_lines.push(line);
            if recent_lines.len() > max_lines as usize {
                recent_lines.remove(0);
            }
        }
    }

    recent_lines
}

/// Print the pixel stuct as color
pub fn print_color(buffer: Vec<Pixel>) {
    for i in 0..buffer.len() {
        print!("{}", color_text("#", buffer[i].r, buffer[i].g, buffer[i].b));
    }
}
// "▄"
/// Print an image in console version using a list of Pixel structs
pub fn print_color_v(buffer: &Vec<Pixel>, width: usize) {
    for i in 0..buffer.len() / 2 {
        print!(
            "{}",
            color(
                "▄",
                buffer[i].r,
                buffer[i].g,
                buffer[i].b,
                buffer[i + width].r,
                buffer[i + width].g,
                buffer[i + width].b
            )
        );
        if i % width == width - 1 {
            print!("\n");
        }
    }
}
