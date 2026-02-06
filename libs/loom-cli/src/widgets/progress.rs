use std::fmt;

use super::{Widget, WidgetResult};

pub struct ProgressBar {
    current: usize,
    total: usize,
    message: String,
    status_icon: Option<char>,
    bar_width: usize,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            current: 0,
            total: 100,
            message: String::new(),
            status_icon: None,
            bar_width: 30,
        }
    }

    pub fn current(mut self, current: usize) -> Self {
        self.current = current;
        self
    }

    pub fn total(mut self, total: usize) -> Self {
        self.total = total;
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn status(mut self, icon: char) -> Self {
        self.status_icon = Some(icon);
        self
    }

    pub fn bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    pub fn clear() {
        super::clear_line();
    }
}

impl Widget for ProgressBar {
    fn render(&self) -> WidgetResult {
        let pct = if self.total > 0 {
            self.current as f32 / self.total as f32
        } else {
            0.0
        };
        let filled = (pct * self.bar_width as f32) as usize;
        let empty = self.bar_width.saturating_sub(filled);
        let status = self
            .status_icon
            .map(|c| format!(" {}", c))
            .unwrap_or_default();

        WidgetResult::new(format!(
            "[{}{}] {:3.0}% ({}/{}){}  {}",
            "█".repeat(filled),
            "░".repeat(empty),
            pct * 100.0,
            self.current,
            self.total,
            status,
            self.message
        ))
    }
}

impl fmt::Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.render())
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}
