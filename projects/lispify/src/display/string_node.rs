use super::*;

pub struct LispStyled {
    text: String,
    style: AnsiStyle,
}

impl PrettyPrint for LispStyled {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        PrettyTree::text(self.text.clone()).annotate()
    }
}

#[derive(Clone, Debug)]
pub struct LispString {
    text: String,
    unit: String,
}

impl From<LispString> for Lisp {
    fn from(value: LispString) -> Self {
        Self::String(Box::new(value))
    }
}

impl PrettyPrint for LispString {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}

impl LispString {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string(), unit: String::new() }
    }
    pub fn with_unit<S: ToString>(self, unit: S) -> Self {
        Self { unit: unit.to_string(), ..self }
    }
}


