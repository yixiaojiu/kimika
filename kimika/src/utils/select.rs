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
use std::fmt::Formatter;
use std::{fmt::Display, io::Write};
use tokio::{select, sync::mpsc};
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct Line {
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
    pub fn not_selected_pointer(&mut self, pointer: char) {
        self.not_selected_pointer = pointer;
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

pub struct Select<I, W>
where
    I: ToString + Display,
    W: Write, // W: std::io::Write, // F: Fn(SelectDialogKey, &I),
{
    items: Vec<SelectItem<I>>,
    lines: Vec<Line>,
    selected_item: usize,
    pointer: char,
    not_selected_pointer: Option<char>,
    default_up: KeyCode,
    default_down: KeyCode,
    up_keys: Vec<KeyCode>,
    down_keys: Vec<KeyCode>,
    move_selected_item_forward: bool,
    underline_selected_item: bool,
    longest_item_len: usize,
    item_count: usize,
    hint_message: Option<&'static str>,
    out: W,
    // out: Option<W>, // logger: Logger<W>,
}

impl<I, W> Select<I, W>
where
    I: ToString + Display + core::fmt::Debug,
    W: std::io::Write,
{
    /// Create a new Select Dialog with lines defined in the items parameter.
    ///
    /// Any Struct that implements std::io::write can be used as output. Use std::io::stdout() as second parameter to print to console
    pub fn new(
        items: Vec<SelectItem<I>>,
        out: W,
        hint_message: Option<&'static str>,
    ) -> Select<I, W> {
        Select {
            items,
            pointer: '>',
            selected_item: 0,
            default_up: Up,
            default_down: Down,
            not_selected_pointer: None,
            move_selected_item_forward: false,
            underline_selected_item: false,
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
            let mut line = Line::new(item.label.to_string(), self.pointer);

            if let Some(pointer) = self.not_selected_pointer {
                line.not_selected_pointer(pointer);
            }

            if line.len() > self.longest_item_len {
                self.longest_item_len = line.len()
            }
            lines.push(line);
            item_count += 1;
        }
        self.lines = lines;
        self.item_count = item_count;
    }
    fn print_lines(&mut self) {
        self.lines.iter_mut().for_each(|line| line.default());

        self.lines[self.selected_item].select();

        if self.underline_selected_item {
            self.lines[self.selected_item].underline();
        }
        if self.move_selected_item_forward {
            self.lines[self.selected_item].space_from_pointer(2);
        }

        for line in self.lines.iter() {
            writeln!(&mut self.out, "{}", line).unwrap()
        }
    }

    fn erase_printed_items(&mut self) {
        utils::crossterm::clear_up_lines((self.item_count) as u16).unwrap();
    }

    fn move_up(&mut self) {
        if self.selected_item == 0 {
            return;
        };
        self.selected_item -= 1;
        self.erase_printed_items();
        self.print_lines();
    }
    fn move_down(&mut self) {
        if self.selected_item == self.items.len() - 1 {
            return;
        }

        self.selected_item += 1;
        self.erase_printed_items();
        self.print_lines();
    }

    pub async fn start_rx(
        &mut self,
        rx: &mut mpsc::Receiver<Vec<SelectItem<I>>>,
    ) -> Option<&SelectItem<I>> {
        self.up_keys.push(self.default_up);
        self.down_keys.push(self.default_down);
        enable_raw_mode().unwrap();
        let mut reader = EventStream::new();

        loop {
            enable_raw_mode().unwrap();
            let event = reader.next();
            let rx_items = rx.recv();

            select! {
                maybe_items = rx_items => {
                    disable_raw_mode().unwrap();
                    if let Some(items) = maybe_items {
                        self.modify_items(items);
                    }
                    enable_raw_mode().unwrap();
                },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            disable_raw_mode().unwrap();
                            if event == Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)) && !self.items.is_empty() {
                                break;
                            }
                            if event == Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)) {
                                return None;
                            }
                            if self.event_contains_key(event.clone(), &self.up_keys) {
                                self.move_up();
                                continue;
                            } else if self.event_contains_key(event, &self.down_keys) {
                                self.move_down();
                                continue;
                            }
                        }
                        Some(Err(e)) => println!("Error: {:?}\r", e),
                        None => break,
                    }
                }
            };
        }

        self.erase_printed_items();
        Some(&self.items[self.selected_item])
    }
    fn event_contains_key(&self, event: Event, keys: &[KeyCode]) -> bool {
        for key in keys.iter() {
            if event == Event::Key(KeyEvent::new(key.clone(), KeyModifiers::NONE)) {
                return true;
            }
        }
        false
    }
    fn modify_items(&mut self, items: Vec<SelectItem<I>>) {
        if items.is_empty() {
            self.erase_printed_items();
            if let Some(hint) = self.hint_message {
                println!("{}", hint.yellow());
            }
            self.items = items;
            self.selected_item = 0;
        } else {
            if self.items.is_empty() && self.hint_message.is_some() {
                utils::crossterm::clear_up_lines(1u16).unwrap();
            }
            self.items = items;
            self.selected_item = 0;
            self.build_lines();
            self.print_lines();
        }
    }
}

pub async fn receiver_select(
    rx: &mut mpsc::Receiver<Vec<SelectItem<String>>>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("Select a receiver >> (Press q to exit)");
    let mut select = Select::new(Vec::new(), std::io::stdout(), Some("Searching receiver..."));
    let select_item = select.start_rx(rx).await;
    if select_item.is_none() {
        return Ok(None);
    }
    let select_item = select_item.unwrap();

    utils::crossterm::clear_up_lines(1u16).unwrap();
    println!("Select a receiver >> {}", select_item.label.clone().cyan());
    Ok(Some(select_item.id.clone()))
}
