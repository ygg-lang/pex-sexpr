use crate::Lisp;
use pretty::{
    termcolor::{Buffer, Color, ColorSpec},
    RcDoc,
};

///
pub fn colored_lisp<T: Into<Lisp>>(object: T, width: usize) -> Result<String, std::io::Error> {
    // let mut w = Vec::new();
    let mut buffer = Buffer::ansi();
    object.into().to_doc().render_colored(width, &mut buffer)?;
    unsafe { Ok(String::from_utf8_unchecked(buffer.into_inner())) }
}

pub fn display_lisp<T: Into<Lisp>>(object: T, width: usize) -> Result<String, std::fmt::Error> {
    // let mut w = Vec::new();
    let mut buffer = String::new();
    object.into().to_doc().render_fmt(width, &mut buffer)?;
    Ok(buffer)
}

pub fn colored_text(text: &str, color: Color) -> RcDoc<ColorSpec> {
    RcDoc::text(text).annotate(ColorSpec::new().set_fg(Some(color)).clone())
}
