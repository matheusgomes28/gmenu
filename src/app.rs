use std::{
    io::{Stderr, Stdout, stdout},
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use ratatui::{
    DefaultTerminal, Frame, Terminal, buffer::Buffer, layout::Rect, prelude::CrosstermBackend, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}
};

use crate::models::MenuItem;

type AppTerminal = Terminal<CrosstermBackend<Stderr>>;

#[derive(Default)]
pub struct App {
    pub title: String,
    pub items: Vec<MenuItem>,
    pub exit: bool,
    pub dmenu: bool,
    pub selected_index: usize,
    pub selected_item: Option<MenuItem>,
}

impl App {
    pub fn run(&mut self, terminal: &mut AppTerminal) -> Result<()> {
        let mut selection = String::new();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            selection = self.handle_events()?;
        }

        disable_raw_mode()?;
        execute!(std::io::stdout(), LeaveAlternateScreen)?;
        println!("{}", selection);
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<String> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };

        if let Some(selected_item) = &self.selected_item {
            // If dmenu, we simply return the selection
            if self.dmenu {
                return Ok(selected_item.name.clone());
            }

            // Else, let's run the command attached to the item
            let err = Command::new(selected_item.command.clone())
                .args(selected_item.args.clone())
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .exec(); // Never returns on success

            // Only reached if exec fails
            eprintln!("Failed to exec: {}", err);
            std::process::exit(1);
        }

        Ok("".to_string())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => self.move_menu_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_menu_up(),
            KeyCode::Enter => self.select_current_option(),
            KeyCode::Char('q') => self.exit(),
            _ => {} // TODO : Log unhandled key event
        }
    }

    fn move_menu_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.items.len();
    }
    fn move_menu_up(&mut self) {
        let new_index = self.selected_index.saturating_sub(1);

        if new_index == self.selected_index {
            self.selected_index = self.items.len() - 1;
        } else {
            self.selected_index = new_index;
        }
    }
    fn select_current_option(&mut self) {
        let selected_item = self.items.get(self.selected_index);

        if selected_item.is_some() {
            self.exit = true;
            self.selected_item = selected_item.cloned();
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(self.title.clone().bold());

        let instructions = Line::from(vec![
            "k, ^ = <Up>".blue().bold(),
            "j, v = <Down>".into(),
            "enter = <Select>".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let max_chars = self.items.iter().map(|l| l.text.len()).max().unwrap_or(0);
        let padded_lines = self
            .items
            .iter()
            .map(|i| i.text.as_str())
            .map(right_pad(max_chars));

        let item_lines = padded_lines
            .enumerate()
            .map(|(i, item)| (Line::from(item), i == self.selected_index))
            .map(|(line, is_selected)| {
                if is_selected {
                    line.blue().bold()
                } else {
                    line
                }
            })
            .collect::<Vec<Line>>();

        let item_text = Text::from(item_lines);

        Paragraph::new(item_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

// Align items on the left by padding items on the right with
// spaces
fn right_pad(max: usize) -> impl Fn(&str) -> String {
    move |l| format!("{}{}", l, " ".repeat(max - l.len()))
}
