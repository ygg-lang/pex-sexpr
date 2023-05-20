use pretty::termcolor::{Color, ColorChoice, ColorSpec, StandardStream};

use std::path::Display;
use pretty::RcDoc;
use pretty::Doc;

///
#[derive(Debug)]
pub enum Lisp {
    List(Vec<Lisp>),
    Number(ListNumber),
}

#[derive(Debug)]
pub struct ListNumber {
    pub number: String,
    pub unit: String,
}

impl From<ListNumber> for Lisp {
    fn from(value: ListNumber) -> Self {
        Lisp::Number(value)
    }
}

impl ListNumber {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        let n = RcDoc::text(&self.number).annotate(ColorSpec::new().set_fg(Some(Color::Red)).clone());
        if self.unit.is_empty() {
            n
        } else {
            let unit = RcDoc::text(&self.unit).annotate(ColorSpec::new().set_fg(Some(Color::Cyan)).clone());
            n.append(unit)
        }
    }
}

impl Lisp {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        match *self {
            Lisp::Number(ref x) => x.to_doc(),
            Lisp::List(ref xs) =>
                RcDoc::text("(")
                    .append(RcDoc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::line()).nest(1).group())
                    .append(RcDoc::text(")"))
        }
    }
}

