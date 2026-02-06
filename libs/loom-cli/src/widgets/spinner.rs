use std::fmt;

use super::{Widget, WidgetResult};

const FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub struct Spinner {
    message: String,
    frame_idx: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            frame_idx: 0,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn frame(mut self, idx: usize) -> Self {
        self.frame_idx = idx % FRAMES.len();
        self
    }

    pub fn tick(&mut self) {
        self.frame_idx = (self.frame_idx + 1) % FRAMES.len();
    }

    pub fn clear() {
        super::clear_line();
    }
}

impl Widget for Spinner {
    fn render(&self) -> WidgetResult {
        WidgetResult::new(format!("{} {}", FRAMES[self.frame_idx], self.message))
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.render())
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
