use crate::Lisp;
use pretty::{
    termcolor::{Color, ColorChoice, ColorSpec, StandardStream},
    RcDoc,
};

pub fn colored_lisp<T: Into<Lisp>>(object: T, width: usize) -> std::io::Result<String> {
    let mut w = Vec::new();
    object.into().to_doc().render_colored(width, StandardStream::stdout(ColorChoice::Auto))?;
    unsafe { Ok(String::from_utf8_unchecked(w)) }
}

pub fn colored_text(text: &str, color: Color) -> RcDoc<ColorSpec> {
    RcDoc::text(text).annotate(ColorSpec::new().set_fg(Some(color)).clone())
}
