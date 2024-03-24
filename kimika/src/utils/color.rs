pub fn paint_green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

pub fn paint_yellow(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

pub enum Color {
    Yellow,
    Green,
}

pub fn print_color(text: &str, color: Color) {
    let paint_text = match color {
        Color::Yellow => paint_yellow(text),
        Color::Green => paint_green(text),
    };
    println!("{}", paint_text);
}
