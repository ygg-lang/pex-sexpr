use crate::Lisp;

pub trait Lispify {
    type Output: Into<Lisp>;
    fn lispify(&self) -> Self::Output;
}

impl Lispify for Lisp {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        self.clone()
    }
}
