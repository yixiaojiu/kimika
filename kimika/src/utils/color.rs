pub fn paint_green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

pub fn paint_yellow(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}
