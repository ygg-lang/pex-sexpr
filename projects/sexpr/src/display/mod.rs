use pretty::termcolor::{Color, ColorChoice, ColorSpec, StandardStream};

use crate::helpers::colored_text;
use pretty::{Doc, RcDoc};
use std::path::Display;

///
#[derive(Debug)]
pub enum Lisp {
    List(Vec<Lisp>),
    Number(Box<LispNumber>),
    Operator(Box<LispOperator>),
}

#[derive(Debug)]
pub struct LispNumber {
    pub number: String,
    pub unit: String,
}

pub struct LispOperator {
    pub operator: String,
    pub rest: Vec<Lisp>,
}

impl From<LispOperator> for Lisp {
    fn from(value: LispOperator) -> Self {
        Lisp::Operator(Box::new(value))
    }
}

impl From<LispNumber> for Lisp {
    fn from(value: LispNumber) -> Self {
        Lisp::Number(Box::new(value))
    }
}

impl LispNumber {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        let n = RcDoc::text(&self.number).annotate(ColorSpec::new().set_fg(Some(Color::Red)).clone());
        if self.unit.is_empty() {
            n
        }
        else {
            let unit = RcDoc::text(&self.unit).annotate(ColorSpec::new().set_fg(Some(Color::Cyan)).clone());
            n.append(unit)
        }
    }
}

impl LispOperator {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        let operator = colored_text(&self.operator, Color::Blue);
        let terms = vec![operator].into_iter().chain(self.rest.iter().map(|x| x.to_doc()));
        let inner = RcDoc::intersperse(terms, Doc::line()).nest(1).group();
        RcDoc::text("(").append(inner).append(RcDoc::text(")"))
    }
}

impl Lisp {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        match self {
            Lisp::Number(x) => x.to_doc(),
            Lisp::List(xs) => RcDoc::text("(")
                .append(RcDoc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::line()).nest(1).group())
                .append(RcDoc::text(")")),
            Lisp::Operator(v) => v.to_doc(),
        }
    }
}
