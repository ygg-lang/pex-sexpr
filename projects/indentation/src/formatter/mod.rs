use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Arguments, Debug, Formatter, Write};
use core::ops::{AddAssign, SubAssign};


/// A wrapper around a `Formatter` that adds indentation.
pub struct IndentFormatter<'a, 'i> {
    raw: &'i mut Formatter<'a>,
    /// The current indentation level.
    indent_level: usize,
    /// The characters to use for indentation.
    indent_chars: &'static str,
}

impl<'a, 'i> Borrow<Formatter<'a>> for IndentFormatter<'a, 'i> {
    fn borrow(&self) -> &Formatter<'a> {
         self.raw
    }
}

impl<'a, 'i> BorrowMut<Formatter<'a>> for IndentFormatter<'a, 'i> {
    fn borrow_mut(&mut self) -> &mut Formatter<'a> {
        self.raw
    }
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
    /// Create a new `IndentFormatter` from a `Formatter` and an indent string.
    pub fn new(f: &'i mut Formatter<'a>, indent: &'static str) -> Self {
        Self {
            raw: f,
            indent_level: 0,
            indent_chars: indent,
        }
    }
    /// Wrap an `IndentDisplay` in an `IndentFormatter`.
    pub fn wrap<T: IndentDisplay>(item: &T, f: &'i mut Formatter<'a>) -> core::fmt::Result {
        item.indent_fmt(&mut IndentFormatter::new(f, "    "))
    }
    /// Unwrap the `IndentFormatter` to get the underlying `Formatter`.
    pub fn unwrap(self) -> &'i mut Formatter<'a> {
        self.raw
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
    /// Write a string, splitting it into lines and indenting each line.
    pub fn write_lines(&mut self, s: &str) -> core::fmt::Result {
        for line in s.lines() {
            self.write_indent()?;
            self.raw.write_str(line)?;
        }
        Ok(())
    }
    /// Add one to the current indentation level.
    pub fn indent(&mut self) {
        self.indent_level = self.indent_level.saturating_add(1);
    }
    /// Subtract one from the current indentation level.
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
}

/// A trait for types that can be displayed with indentation.
pub trait IndentDisplay {
    /// Display the type with indentation.
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result;
}
