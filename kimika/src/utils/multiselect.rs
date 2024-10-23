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
use std::{fmt::Display, io::Write};
use std::{fmt::Formatter, io};
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
    default_up: KeyCode,
    default_down: KeyCode,
    /// Keys that should move the selected item forward
    up_keys: Vec<KeyCode>,
    /// Keys that should move the selected item backward
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
    ///         
    /// - `hint_message` - display hint message when there is no option
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

    /// clear all printed lines
    fn erase_printed_items(&mut self) -> Result<(), io::Error> {
        if self.item_count != 0 {
            utils::crossterm::clear_up_lines((self.item_count) as u16)?;
        }
        Ok(())
    }

    fn move_up(&mut self) -> Result<(), io::Error> {
        if self.selected_item == 0 {
            return Ok(());
        };
        self.selected_item -= 1;
        self.erase_printed_items()?;
        self.print_lines();
        Ok(())
    }
    fn move_down(&mut self) -> Result<(), io::Error> {
        if self.selected_item == self.items.len() - 1 {
            return Ok(());
        }

        self.selected_item += 1;
        self.erase_printed_items()?;
        self.print_lines();
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
            println!("{}", self.hint_message.unwrap().yellow());
        } else {
            self.build_lines();
            self.print_lines();
        }

        enable_raw_mode()?;
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
                            if event == Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)) {
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
        Ok(Some(&self.items[self.selected_item]))
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
                utils::crossterm::clear_up_lines(1u16).unwrap();
            } else {
                self.erase_printed_items()?;
            }
            if let Some(hint) = self.hint_message {
                println!("{}", hint.yellow());
            }
            self.items = items;
            self.selected_item = 0;
        } else {
            if self.items.is_empty() && self.hint_message.is_some() {
                if self.hint_message.is_some() {
                    utils::crossterm::clear_up_lines(1u16).unwrap();
                }
            } else {
                self.erase_printed_items()?;
            }
            self.items = items;
            self.selected_item = 0;
            self.build_lines();
            self.print_lines();
        }
        Ok(())
    }
}

pub async fn metadata_select() -> Result<(), Box<dyn std::error::Error>> {
    println!("======================");

    let options = vec![utils::select::SelectItem {
        id: "1000000".to_string(),
        label: "1000000".to_string(),
    }];

    let mut select = utils::select::Select::new(options, io::stderr(), None);
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        let mut num = 1000000;

        let mut options = vec![];

        for _ in 1..5 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            num += 1000000;
            options.push(utils::select::SelectItem {
                id: num.to_string(),
                label: num.to_string(),
            });

            tx.send(options.clone()).await.unwrap();
        }
    });

    if let Some(select) = select.start_rx(&mut rx).await? {
        println!("{}", select.label)
    } else {
        println!("none")
    }

    Ok(())
}
