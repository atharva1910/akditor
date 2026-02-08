enum CursorFrame {
    Status,
    File(u16),
}

pub struct Cursor {
    frame: CursorFrame,
}
