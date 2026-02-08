mod frames;
mod logger;
mod cursor;
mod editor;
mod modifiers;
use crate::editor::Editor;


fn main() {
    let mut term = ratatui::init();
    Editor::new().run(&mut term);
    ratatui::restore();
}
