mod frames;
mod logger;
mod editor;
mod modifiers;
use crate::editor::Editor;


fn main() {
    let mut term = ratatui::init();
    Editor::new(term.get_frame().area()).run(&mut term);
    ratatui::restore();
}
