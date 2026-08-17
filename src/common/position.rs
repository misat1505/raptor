use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: usize,
    pub filename: Option<&'static str>,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "{}{}:{}",
            match self.filename {
                Some(filename) => format!("{}:", filename),
                None => String::new(),
            },
            self.line,
            self.column
        )?)
    }
}

impl Position {
    pub fn new(line: u32, column: u32, offset: usize, filename: Option<&'static str>) -> Self {
        Position {
            line,
            column,
            offset,
            filename,
        }
    }

    pub fn location(&self) -> String {
        let file = self.filename.unwrap_or("<input>");
        return format!("{}:{}:{}", file, self.line, self.column);
    }
}
