mod progress;
mod spinner;
mod table;

use std::io::{Write, stdout};
use std::ops::Deref;

use crossterm::{ExecutableCommand, cursor, terminal};

pub use progress::ProgressBar;
pub use spinner::Spinner;
pub use table::Table;

/// Result of rendering a widget, wraps the rendered string
pub struct WidgetResult(String);

impl Deref for WidgetResult {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WidgetResult {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Write to stdout, clearing the current line first
    pub fn write(&self) {
        self.write_to(&mut stdout());
    }

    /// Write to any writer, clearing the line first (for terminal writers)
    pub fn write_to(&self, writer: &mut impl Write) {
        let mut stdout = stdout();
        let _ = stdout.execute(cursor::MoveToColumn(0));
        let _ = stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine));
        let _ = write!(writer, "{}", self.0);
        let _ = writer.flush();
    }
}

/// Trait for all widgets that can be rendered
pub trait Widget: std::fmt::Display {
    fn render(&self) -> WidgetResult;
}

/// Clear the current line (useful after inline widgets)
pub fn clear_line() {
    let mut stdout = stdout();
    let _ = stdout.execute(cursor::MoveToColumn(0));
    let _ = stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine));
    let _ = stdout.flush();
}
