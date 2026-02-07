mod frames;
mod logger;
mod cursor;
mod editor;
mod modifiers;
use crate::editor::Editor;
use crossterm::event;


fn main() {
    let mut term = ratatui::init();
    let mut editor = Editor::new(term.size().unwrap());

    while !editor.quit {
        editor.update();
        let _ = term.draw(|f| editor.draw(f));

        if let Ok(event) = event::read() {
            editor.handle_event(event);
        }
    }
    ratatui::restore();
}
