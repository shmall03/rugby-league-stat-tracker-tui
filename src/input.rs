use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct TextInput {
    pub buffer: String,
    pub active: bool,
    pub prompt: String,
    cursor_pos: usize,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            buffer: String::new(),
            active: false,
            prompt: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn start(&mut self, prompt: &str) {
        self.buffer.clear();
        self.active = true;
        self.prompt = prompt.to_string();
        self.cursor_pos = 0;
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        if !self.active {
            return None;
        }
        match key.code {
            KeyCode::Enter => {
                let result = self.buffer.clone();
                self.active = false;
                Some(result)
            }
            KeyCode::Esc => {
                self.active = false;
                None
            }
            KeyCode::Char(c) => {
                self.buffer.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
                None
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.cursor_pos = self.cursor_pos.saturating_sub(1);
                    self.buffer.remove(self.cursor_pos);
                }
                None
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.buffer.len() {
                    self.buffer.remove(self.cursor_pos);
                }
                None
            }
            KeyCode::Left => {
                self.cursor_pos = self.cursor_pos.saturating_sub(1);
                None
            }
            KeyCode::Right => {
                self.cursor_pos = self.cursor_pos.min(self.buffer.len());
                None
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
                None
            }
            KeyCode::End => {
                self.cursor_pos = self.buffer.len();
                None
            }
            _ => None,
        }
    }

}
