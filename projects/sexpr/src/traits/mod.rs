use crate::Lisp;

pub trait Lispify {
    type Output: Into<Lisp>;
    fn lispify(&self) -> Self::Output;
}