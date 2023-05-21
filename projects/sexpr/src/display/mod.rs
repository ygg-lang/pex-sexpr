use pretty::termcolor::{Color, ColorSpec};

use crate::{helpers::colored_text, Lispify};
use pretty::{Doc, RcDoc};

///
#[derive(Debug)]
pub enum Lisp {
    Any(Vec<Lisp>),
    Keyword(String),
    Function(String),
    Operator(String),
    Number(Box<LispNumber>),
    Symbol(Box<LispSymbol>),
    String(Box<ListString>),
}

#[derive(Debug)]
pub struct ListString {
    pub text: String,
    pub unit: String,
}
impl From<ListString> for Lisp {
    fn from(value: ListString) -> Self {
        Lisp::String(Box::new(value))
    }
}
#[derive(Debug)]
pub struct LispNumber {
    pub number: String,
    pub unit: String,
}

impl From<LispSymbol> for Lisp {
    fn from(value: LispSymbol) -> Self {
        Lisp::Symbol(Box::new(value))
    }
}

#[derive(Debug)]
pub struct LispSymbol {
    pub name: String,
    pub path: Vec<String>,
}

impl From<LispNumber> for Lisp {
    fn from(value: LispNumber) -> Self {
        Lisp::Number(Box::new(value))
    }
}

impl LispSymbol {
    pub fn new<S: ToString>(name: S) -> Self {
        LispSymbol { name: name.to_string(), path: vec![] }
    }

    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        let mut term = colored_text(&self.name, Color::Red);
        for symbol in &self.path {
            let new = colored_text(symbol, Color::Red);
            term = term.append(RcDoc::text("∷")).append(new)
        }
        term
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

impl ListString {
    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        let n = colored_text(&self.text, Color::Green);
        if self.unit.is_empty() {
            n
        }
        else {
            let unit = colored_text(&self.unit, Color::Cyan);
            unit.append(n)
        }
    }
}

impl Lisp {
    pub fn keyword<S: ToString>(name: S) -> Self {
        Lisp::Keyword(name.to_string())
    }
    pub fn function<S: ToString>(name: S) -> Self {
        Lisp::Function(name.to_string())
    }
    pub fn operator<S: ToString, T: Lispify>(name: S, arguments: &[T]) -> Self {
        let head = Lisp::Operator(name.to_string());
        let mut terms = Vec::with_capacity(arguments.len() + 1);
        terms.push(head);
        terms.extend(arguments.iter().map(|x| x.lispify().into()));
        Lisp::Any(terms)
    }

    pub(crate) fn to_doc(&self) -> RcDoc<ColorSpec> {
        match self {
            Lisp::Any(xs) => RcDoc::text("(")
                .append(RcDoc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::line()).nest(2).group())
                .append(RcDoc::text(")")),
            Lisp::Keyword(v) => colored_text(v, Color::Magenta),
            Lisp::Operator(v) => colored_text(v, Color::Blue),
            Lisp::Function(v) => colored_text(v, Color::Blue),
            Lisp::Symbol(v) => v.to_doc(),
            Lisp::String(v) => v.to_doc(),
            Lisp::Number(x) => x.to_doc(),
        }
    }
}
