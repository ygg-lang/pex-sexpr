use core::fmt::{Arguments, Debug, Formatter, Write};
use core::ops::{AddAssign, SubAssign};



pub struct IndentFormatter<'a, 'i> {
    pub(crate) raw: &'i mut Formatter<'a>,
    pub indent_level: usize,
    pub indent_chars: &'i str,
}

impl<'a, 'i> Debug for IndentFormatter<'a, 'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IndentFormatter")
            .field("indent_level", &self.indent_level)
            .field("indent_chars", &self.indent_chars)
            .field("width", &self.raw.width())
            .finish()
    }
}

impl AddAssign<usize> for IndentFormatter<'_, '_> {
    fn add_assign(&mut self, rhs: usize) {
        self.indent_level = self.indent_level.saturating_add(rhs);
    }
}

impl SubAssign<usize> for IndentFormatter<'_, '_> {
    fn sub_assign(&mut self, rhs: usize) {
        self.indent_level = self.indent_level.saturating_sub(rhs)
    }
}

impl Write for IndentFormatter<'_, '_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.raw.write_str(s)
    }
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.raw.write_char(c)
    }
    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        self.raw.write_fmt(args)
    }
}


impl<'a, 'i> IndentFormatter<'a, 'i> {
    pub fn new(f: &'i mut Formatter<'a>, indent: &'i str) -> Self {
        Self {
            raw: f,
            indent_level: 0,
            indent_chars: indent,
        }
    }
    /// Write the current indentation level to the formatter.
    pub fn write_indent(&mut self) -> core::fmt::Result {
        for _ in 0..self.indent_level {
            self.raw.write_str(self.indent_chars)?;
        }
        Ok(())
    }
    /// Write a newline and the current indentation level to the formatter.
    pub fn write_newline(&mut self) -> core::fmt::Result {
        self.raw.write_char('\n')?;
        self.write_indent()?;
        Ok(())
    }
    pub fn write_lines(&mut self, s: &str) -> core::fmt::Result {
        for line in s.lines() {
            self.write_indent()?;
            self.raw.write_str(line)?;
        }
        Ok(())
    }
    pub fn indent(&mut self) {
        self.indent_level = self.indent_level.saturating_add(1);
    }
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
}

pub trait IndentDisplay {
    fn indent_fmt(&self, f: IndentFormatter) -> core::fmt::Result;
}
