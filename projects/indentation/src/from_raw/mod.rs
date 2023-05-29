use core::fmt::Formatter;
use crate::IndentFormatter;

impl<'a, 'i> From<&'i mut Formatter<'a>> for IndentFormatter<'a, 'i> {
    fn from(f: &'i mut Formatter<'a>) -> Self {
        Self::new(f, "    ")
    }
}
