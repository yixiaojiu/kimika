use crate::utils;
/// refrence https://github.com/Preyde/cli_select/
use core::fmt;
use crossterm::{
    event::{
        Event, EventStream,
        KeyCode::{self, Down, Up},
        KeyEvent, KeyModifiers,
    },
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{fmt::Formatter, io};
use std::{
    fmt::{Debug, Display},
    io::Write,
};
use tokio::{select, sync::mpsc};
use tokio_stream::StreamExt;

#[derive(Debug)]
struct Line {
    text: String,
    is_selected: bool,
    pointer: char,
    not_selected_pointer: char,
    space: usize,
    underline: bool,
}

impl Line {
    pub fn new(text: String, pointer: char) -> Self {
        Line {
            text,
            is_selected: false,
            pointer,
            space: 1,
            underline: false,
            not_selected_pointer: ' ',
        }
    }
    pub fn select(&mut self) {
        self.is_selected = true;
    }
    pub fn underline(&mut self) {
        self.underline = true;
    }
    /// Define the space between pointer and item. Default is 1.
    pub fn space_from_pointer(&mut self, space: usize) {
        self.space = space;
    }
    /// set all changes back to default that were made after creation
    pub fn default(&mut self) {
        self.is_selected = false;
        self.space = 1;
        self.underline = false;
    }
    /// ascii code to underline
    fn underline_text(&self, text: &str) -> String {
        format!("\x1b[4m{}\x1b[0m", text)
    }
    pub fn len(&self) -> usize {
        self.text.chars().count() + self.space + 1
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = if self.underline {
            Some(self.underline_text(&self.text))
        } else {
            None
        };
        let pointer = if self.is_selected {
            self.pointer
        } else {
            self.not_selected_pointer
        };

        let result = format!(
            "{}{}{}",
            pointer,
            " ".repeat(self.space),
            text.as_ref().unwrap_or(&self.text),
        );

        write!(
            f,
            "{}",
            if self.is_selected {
                result.green()
            } else {
                result.white()
            }
        )?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct SelectItem<I: ToString + Display> {
    pub id: String,
    pub label: I,
}

impl<I: ToString + Display> SelectItem<I> {
    pub fn new(id: String, label: I) -> Self {
        SelectItem { id, label }
    }
}

pub struct ReceiverSelect<I, W>
where
    I: ToString + Display,
    W: Write,
{
    items: Vec<SelectItem<I>>,
    lines: Vec<Line>,
    selected_index: usize,
    pointer: char,
    default_up: KeyCode,
    default_down: KeyCode,
    /// Keys that should move the selected item forward
    up_keys: Vec<KeyCode>,
    /// Keys that should move the selected item backward
    down_keys: Vec<KeyCode>,
    move_selected_item_forward: bool,
    longest_item_len: usize,
    item_count: usize,
    hint_message: Option<&'static str>,
    out: W,
}

impl<I, W> ReceiverSelect<I, W>
where
    I: ToString + Display + core::fmt::Debug,
    W: std::io::Write,
{
    /// Create a new Select Dialog with lines defined in the items parameter.
    ///
    /// Any Struct that implements std::io::write can be used as output. Use std::io::stdout() as second parameter to print to console
    ///         
    /// - `hint_message` - display hint message when there is no option
    pub fn new(
        items: Vec<SelectItem<I>>,
        out: W,
        hint_message: Option<&'static str>,
    ) -> ReceiverSelect<I, W> {
        ReceiverSelect {
            items,
            pointer: '>',
            selected_index: 0,
            default_up: Up,
            default_down: Down,
            move_selected_item_forward: false,
            up_keys: vec![],
            down_keys: vec![],
            lines: vec![],
            longest_item_len: 0,
            item_count: 0,
            hint_message,
            out,
        }
    }
    /// Builds the lines and store them for later usage. item_count and longest_item_len is initialized.
    fn build_lines(&mut self) {
        let mut lines: Vec<Line> = vec![];
        let mut item_count: usize = 0;
        for item in &self.items {
            let line = Line::new(item.label.to_string(), self.pointer);

            if line.len() > self.longest_item_len {
                self.longest_item_len = line.len()
            }
            lines.push(line);
            item_count += 1;
        }
        self.lines = lines;
        self.item_count = item_count;
    }
    fn print_lines(&mut self) -> Result<(), io::Error> {
        self.lines.iter_mut().for_each(|line| line.default());

        self.lines[self.selected_index].select();

        if self.move_selected_item_forward {
            self.lines[self.selected_index].space_from_pointer(2);
        }

        writeln!(&mut self.out, "")?;
        for line in self.lines.iter() {
            writeln!(&mut self.out, "{}", line)?;
        }

        self.set_cursor(self.item_count as u16 + 1)?;

        Ok(())
    }

    /// clear all printed lines
    fn erase_printed_items(&mut self) -> Result<(), io::Error> {
        if self.item_count != 0 {
            // utils::crossterm::clear_up_lines((self.item_count) as u16)?;
            self.clear_lines()?
        }
        Ok(())
    }

    fn move_up(&mut self) -> Result<(), io::Error> {
        if self.selected_index == 0 {
            return Ok(());
        };
        self.selected_index -= 1;
        self.erase_printed_items()?;
        self.print_lines()?;
        Ok(())
    }
    fn move_down(&mut self) -> Result<(), io::Error> {
        if self.selected_index == self.items.len() - 1 {
            return Ok(());
        }

        self.selected_index += 1;
        self.erase_printed_items()?;
        self.print_lines()?;
        Ok(())
    }

    fn set_cursor(&self, row_offset: u16) -> Result<(), io::Error> {
        let (_, row) = crossterm::cursor::position()?;
        crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::MoveTo(30, row - row_offset)
        )?;
        Ok(())
    }

    fn clear_lines(&mut self) -> Result<(), io::Error> {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
        )?;
        Ok(())
    }

    pub async fn start_rx(
        &mut self,
        rx: &mut mpsc::Receiver<Vec<SelectItem<I>>>,
    ) -> Result<Option<&SelectItem<I>>, io::Error> {
        self.up_keys.push(self.default_up);
        self.down_keys.push(self.default_down);
        let mut reader = EventStream::new();

        if self.items.is_empty() && self.hint_message.is_some() {
            writeln!(&mut self.out, "{}", self.hint_message.unwrap().yellow())?;
        } else {
            self.build_lines();
            self.print_lines()?;
        }

        enable_raw_mode()?;
        self.set_cursor(2)?;
        loop {
            let event = reader.next();
            let rx_items = rx.recv();

            select! {
                maybe_items = rx_items => {
                    if let Some(items) = maybe_items {
                        self.modify_items(items)?;
                    }
                }
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if event == Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)) && !self.items.is_empty() {
                                break;
                            }
                            if event == Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
                                || event == Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)) {
                                self.erase_printed_items()?;
                                disable_raw_mode()?;
                                return Ok(None);
                            }
                            if self.event_contains_key(event.clone(), &self.up_keys) {
                                self.move_up()?;
                                continue;
                            } else if self.event_contains_key(event, &self.down_keys) {
                                self.move_down()?;
                                continue;
                            }
                        }
                        Some(Err(e)) => return Err(e),
                        None => break,
                    }
                }
            };
        }

        disable_raw_mode()?;
        self.erase_printed_items()?;
        Ok(Some(&self.items[self.selected_index]))
    }
    fn event_contains_key(&self, event: Event, keys: &[KeyCode]) -> bool {
        for key in keys.iter() {
            if event == Event::Key(KeyEvent::new(key.clone(), KeyModifiers::NONE)) {
                return true;
            }
        }
        false
    }
    fn modify_items(&mut self, items: Vec<SelectItem<I>>) -> Result<(), io::Error> {
        if items.is_empty() {
            if self.items.is_empty() {
                utils::crossterm::clear_up_lines(1u16)?;
            } else {
                self.erase_printed_items()?;
            }
            if let Some(hint) = self.hint_message {
                writeln!(&mut self.out, "{}", hint.yellow())?;
            }
            self.items = items;
            self.selected_index = 0;
        } else {
            if self.items.is_empty() && self.hint_message.is_some() {
                // if self.hint_message.is_some() {
                //     utils::crossterm::clear_up_lines(1u16)?
                // }
                self.clear_lines()?;
            } else {
                self.erase_printed_items()?;
            }
            self.items = items;
            self.build_lines();
            self.print_lines()?;
        }
        Ok(())
    }
}

pub async fn receiver_select(
    rx: &mut mpsc::Receiver<Vec<SelectItem<String>>>,
) -> Result<Option<SelectItem<String>>, io::Error> {
    println!("{} Select a receiver >>", "?".green());
    let mut select =
        ReceiverSelect::new(Vec::new(), std::io::stdout(), Some("Searching receiver..."));
    let select_item = select.start_rx(rx).await?;

    match select_item {
        Some(select_item) => {
            utils::crossterm::clear_up_lines(1u16)?;
            println!(
                "{} Select a receiver >> {}",
                "?".green(),
                select_item.label.clone().cyan()
            );
            Ok(Some(select_item.clone()))
        }
        None => Ok(None),
    }
}

pub async fn select_test() -> Result<(), io::Error> {
    println!("{} Select a receiver >>", "?".green());
    let mut select = ReceiverSelect::new(Vec::new(), std::io::stdout(), Some("dfafafjalfjaofa"));

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        let mut vec = vec![SelectItem::new("1".to_string(), "1".to_string())];
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        tx.send(vec.clone()).await.unwrap();
        for i in 2..7 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            vec.push(SelectItem::new(i.to_string(), i.to_string()));
            tx.send(vec.clone()).await.unwrap();
        }
    });

    select.start_rx(&mut rx).await?;

    Ok(())
}
