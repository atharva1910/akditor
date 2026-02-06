use crate::{
    frames:: FramesFn,
    cursor::{CursorMove, Cursor},
    events::AKEvent,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Block, Borders},
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

pub struct ListBuffers {
}

impl ListBuffers {
}
