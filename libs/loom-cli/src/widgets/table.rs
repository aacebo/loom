use std::fmt;

use super::{Widget, WidgetResult};

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<usize>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            column_widths: Vec::new(),
        }
    }

    pub fn headers(mut self, headers: Vec<impl Into<String>>) -> Self {
        self.headers = headers.into_iter().map(|h| h.into()).collect();
        self.update_column_widths();
        self
    }

    pub fn row(mut self, row: Vec<impl Into<String>>) -> Self {
        self.rows.push(row.into_iter().map(|c| c.into()).collect());
        self.update_column_widths();
        self
    }

    pub fn rows(mut self, rows: Vec<Vec<impl Into<String> + Clone>>) -> Self {
        for row in rows {
            self.rows.push(row.into_iter().map(|c| c.into()).collect());
        }
        self.update_column_widths();
        self
    }

    fn update_column_widths(&mut self) {
        let num_cols = self
            .headers
            .len()
            .max(self.rows.iter().map(|r| r.len()).max().unwrap_or(0));

        self.column_widths.resize(num_cols, 0);

        for (i, header) in self.headers.iter().enumerate() {
            self.column_widths[i] = self.column_widths[i].max(header.len());
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < self.column_widths.len() {
                    self.column_widths[i] = self.column_widths[i].max(cell.len());
                }
            }
        }
    }
}

impl Widget for Table {
    fn render(&self) -> WidgetResult {
        let mut output = String::new();

        // Render headers
        for (i, header) in self.headers.iter().enumerate() {
            let width = self.column_widths.get(i).copied().unwrap_or(header.len());
            output.push_str(&format!("{:>width$} ", header, width = width));
        }
        output.push('\n');

        // Render separator
        let total_width: usize =
            self.column_widths.iter().sum::<usize>() + self.column_widths.len();
        output.push_str(&"-".repeat(total_width));
        output.push('\n');

        // Render rows
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                let width = self.column_widths.get(i).copied().unwrap_or(cell.len());
                output.push_str(&format!("{:>width$} ", cell, width = width));
            }
            output.push('\n');
        }

        WidgetResult::new(output)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.render())
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
