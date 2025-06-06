use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind }, terminal};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,

};
fn main() {
    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();

}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(render)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    Paragraph::new("Hello").render(frame.area(), frame.buffer_mut());
}