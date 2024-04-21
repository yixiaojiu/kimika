use crossterm::{cursor, execute, terminal};

pub fn clear_up_lines(lines: u16) -> Result<(), std::io::Error> {
    execute!(
        std::io::stdout(),
        cursor::MoveToPreviousLine(lines),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )
}
