use crate::Lisp;

/// A trait for converting a type into a lisp data structure
pub trait Lispify {
    /// The output type
    type Output: Into<Lisp>;
    /// Convert `self` into a lisp data structure
    fn lispify(&self) -> Self::Output;
}

impl Lispify for Lisp {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        self.clone()
    }
}
