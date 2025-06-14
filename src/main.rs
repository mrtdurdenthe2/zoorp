use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind }, terminal};
use ratatui::{
    buffer::Buffer,
    layout::{self, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Borders},
    DefaultTerminal, Frame,
    prelude::*,
};


#[derive(Default)]
struct App { // Keep the "data" for the app in here 
    should_quit: bool
}
struct MyHeaderWidget {

}

fn ui (frame: &mut Frame, _app: &App) { // used in impl App::Run, which is used by main
    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(&vec![ 
        Constraint::Percentage(50),
        Constraint::Percentage(50)
    ])
    .split(frame.area());


    let left_pane = Block::default().title_top("LEFT PANE").borders(Borders::ALL);
    let right_pane = Block::default().title_top("RIGHT PANE").borders(Borders::ALL);
    frame.render_widget(left_pane, chunks[0]);
    frame.render_widget(right_pane, chunks[1]);
    }
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();

    app_result
}

impl App {
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
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            _ => {}
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
