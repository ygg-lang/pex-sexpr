use core::fmt::Formatter;
use core::ops::Deref;
use crate::IndentFormatter;

impl<'a, 'i> From<&'i mut Formatter<'a>> for IndentFormatter<'a,'i> {
    fn from(f: &'i mut Formatter<'a>) -> Self {
        Self::new(f, "    ")
    }
}

impl<'a, 'i> Deref for IndentFormatter<'a,'i> {
    type Target = Formatter<'a>;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}