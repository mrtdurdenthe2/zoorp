use std::{collections, default, io};

use collecter::daily_data;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use ratatui::{
    buffer::Buffer,
    layout::{self, Layout, Rect},
    prelude::*,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use reqwest::Result;

mod collecter;

#[derive(Default)]
enum timeframes {
    Hourly,
    #[default]
    Daily,
    Weekly,
    Monthy,
}

#[derive(Default)]
struct timeframe {
    kind: timeframes,
    value: String,
}

#[derive(Default)]
struct App {
    // Keep the core functional data for the app in here
    should_quit: bool,
    current_tf: timeframe,
    current_sbol: String,
    data: String,
}

fn ui(frame: &mut Frame, _app: &App) {
    // used in impl App::Run, which is used by main
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(&vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    let left_pane = Block::default()
        .title_top("LEFT PANE")
        .borders(Borders::ALL);
    let right_pane = Block::default()
        .title_top("RIGHT PANE")
        .borders(Borders::ALL);
    frame.render_widget(left_pane, chunks[0]);
    frame.render_widget(right_pane, chunks[1]);

    let mut data_return = Text::raw(String::from("{}"));
}
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    fn set_chart_data(&mut self) -> Result<(), reqwest::Error> {
        let newdata = collecter::daily_data("TIME_SERIES_DAILY", "TSLA")?;
        Ok(())
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        ui(frame, self);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), reqwest::Error> {
        match key_event.code {
            KeyCode::Esc => {
                self.exit();
                Ok(())
            }

            KeyCode::Char(d) => {
                self.set_chart_data();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn exit(&mut self) {
        self.should_quit = true
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ...
        Paragraph::new("Header text").render(Rect::new(0, 0, area.width, 1), buf);
    }
}
