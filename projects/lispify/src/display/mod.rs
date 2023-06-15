use std::borrow::Cow;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign};
use pretty_print::{AnsiStyle, PrettyPrint, PrettyProvider, PrettyTree};
use pretty_print::helpers::PrettySequence;

pub mod builder;

use pretty_print::PrettyPrintKind;

/// The lisp data structure
#[derive(Clone, Debug)]
pub enum Lisp {
    Atomic(Box<LispStyled>),
    Concat(Vec<Lisp>),
    Sequence(Vec<Lisp>),
}

#[derive(Clone, Debug)]
pub struct LispStyled {
    text: Cow<'static, str>,
    style: PrettyPrintKind,
}

impl PrettyPrint for Lisp {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Lisp::Atomic(value) => {
                value.pretty(theme)
            }
            Lisp::Concat(values) => {
                let mut result = theme.concat();
                for value in values {
                    result = result.add(value.pretty(theme));
                }
                result
            }
            Lisp::Sequence(values) => {
                let mut terms = PrettySequence::new(values.len());
                for value in values {
                    terms = terms.add(value.pretty(theme));
                }
            }
        }
    }
}


impl PrettyPrint for LispStyled {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.style {
            PrettyPrintKind::Normal => {
                theme.text(self.get_text())
            }
            PrettyPrintKind::Keyword => {
                theme.keyword(self.get_text())
            }
            PrettyPrintKind::String => {
                theme.string(self.get_text())
            }
            PrettyPrintKind::Number => {
                theme.number(self.get_text())
            }
            PrettyPrintKind::Annotation => {
                theme.annotation(self.get_text())
            }
            PrettyPrintKind::Argument => {
                theme.argument(self.get_text(), false)
            }
            PrettyPrintKind::ArgumentMutable => {
                theme.argument(self.get_text(), true)
            }
            PrettyPrintKind::Local => {
                theme.variable(self.get_text(), false)
            }
            PrettyPrintKind::LocalMutable => {
                theme.variable(self.get_text(), true)
            }
            PrettyPrintKind::Operator => {
                theme.operator(self.get_text())
            }
            PrettyPrintKind::Structure => {
                theme.structure(self.get_text())
            }
            PrettyPrintKind::Class => {
                theme.structure(self.get_text())
            }
            PrettyPrintKind::Union => {
                theme.structure(self.get_text())
            }
            PrettyPrintKind::UnionDisjoint => {
                theme.structure(self.get_text())
            }
            PrettyPrintKind::Variant => {
                theme.argument(self.get_text(), false)
            }
            PrettyPrintKind::Interface => {
                theme.interface(self.get_text())
            }
            PrettyPrintKind::Trait => {
                theme.interface(self.get_text())
            }
        }
    }
}

impl Default for Lisp {
    fn default() -> Self {
        Self::Sequence(Vec::new())
    }
}

impl<T> AddAssign<T> for Lisp where T: Into<Lisp> {
    fn add_assign(&mut self, rhs: T) {
        match self {
            Lisp::Sequence(list) => list.push(rhs.into()),
            _ => {
                let mut list = vec![self.clone(), rhs.into()];
                *self = Lisp::Sequence(list);
            }
        }
    }
}

impl<T> BitAndAssign<T> for Lisp where T: Into<Lisp> {
    fn bitand_assign(&mut self, rhs: T) {
        match self {
            Lisp::Concat(list) => list.push(rhs.into()),
            _ => {
                let mut list = vec![self.clone(), rhs.into()];
                *self = Lisp::Concat(list);
            }
        }
    }
}

impl<T> Add<T> for Lisp where T: Into<Lisp> {
    type Output = Lisp;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> BitAnd<T> for Lisp where T: Into<Lisp> {
    type Output = Lisp;

    fn bitand(mut self, rhs: T) -> Self::Output {
        self &= rhs;
        self
    }
}

