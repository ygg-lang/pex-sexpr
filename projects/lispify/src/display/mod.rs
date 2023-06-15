pub mod builder;
mod display;

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::VecDeque,
};
use core::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, BitAnd, BitAndAssign},
};
use pretty_print::{
    helpers::{PrettyPrintKind, PrettySequence},
    PrettyBuilder, PrettyPrint, PrettyProvider, PrettyTree,
};
/// The lisp data structure
#[derive(Clone, Debug)]
pub enum Lisp {
    /// A single atom
    Atomic(Box<LispStyled>),
    /// A list of atoms
    Concat(VecDeque<Lisp>),
    /// A list of lists
    Sequence(VecDeque<Lisp>),
}

/// A single atom
#[derive(Clone, Debug)]
pub struct LispStyled {
    text: Cow<'static, str>,
    style: PrettyPrintKind,
}

impl Default for Lisp {
    fn default() -> Self {
        Self::Sequence(VecDeque::new())
    }
}

impl<T> AddAssign<T> for Lisp
where
    T: Into<Lisp>,
{
    fn add_assign(&mut self, rhs: T) {
        match self {
            Lisp::Sequence(list) => list.push_back(rhs.into()),
            _ => {
                let mut list = VecDeque::with_capacity(2);
                list.push_back(self.clone());
                list.push_back(rhs.into());
                *self = Lisp::Sequence(list);
            }
        }
    }
}

impl<T> BitAndAssign<T> for Lisp
where
    T: Into<Lisp>,
{
    fn bitand_assign(&mut self, rhs: T) {
        match self {
            Lisp::Concat(list) => list.push_back(rhs.into()),
            _ => {
                let mut list = VecDeque::with_capacity(2);
                list.push_back(self.clone());
                list.push_back(rhs.into());
                *self = Lisp::Concat(list);
            }
        }
    }
}

impl<T> Add<T> for Lisp
where
    T: Into<Lisp>,
{
    type Output = Lisp;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> BitAnd<T> for Lisp
where
    T: Into<Lisp>,
{
    type Output = Lisp;

    fn bitand(mut self, rhs: T) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<T> FromIterator<T> for Lisp
where
    T: Into<Lisp>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Lisp::Sequence(iter.into_iter().map(|x| x.into()).collect())
    }
}
