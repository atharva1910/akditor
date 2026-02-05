use crossterm::event::{KeyEvent, KeyCode};

#[derive(Copy, Clone)]
pub enum Modifiers {
    CTRL_X,
    CTRL_C,
//    CTRL_F,
}

impl Modifiers {
    pub fn handle_modifier_fn(&self, key: KeyEvent) {
        match self {
            Modifiers::CTRL_X => self.handle_ctrl_x(key),
            Modifiers::CTRL_C => self.handle_ctrl_c(key),
        }
    }

    fn handle_ctrl_x(&self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    _ => panic!("Modifier notsupported {:?}", key ),
                }
            }
            _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
        }
    }

    fn handle_ctrl_c(&self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    _ => panic!("Modifier notsupported {:?}", key ),
                }
            }
            _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
        }
    }
}
