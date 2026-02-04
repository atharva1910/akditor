mod frames;
mod logger;
mod file_frame;
mod cursor;
mod editor;
mod ctrl;
mod status_bar;

use crate::editor::Editor;
use frames::FramesFn;
use crossterm::event;


fn main() {
    let mut term = ratatui::init();
    let mut editor = Editor::new(term.size().unwrap());

    while !editor.quit() {
        term.draw(|f| editor.draw(f));

        if let Ok(event) = event::read() {
            editor.handle_event(event);
        }
    }
    ratatui::restore();
}
