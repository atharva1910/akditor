use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
};

pub trait FramesFn {
    fn render(&self, area: Rect, buf: &mut Buffer);

    fn handle_key_event(&mut self, key: KeyEvent);

    fn quit(&self) -> bool;

    fn resize(&mut self, area: Rect) {
    }

    fn get_cursor_pos(&self) -> Position {
        Position {
            x: 0,
            y: 0,
        }
    }
}
