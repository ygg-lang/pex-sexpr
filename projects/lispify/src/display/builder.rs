use super::*;

impl From<LispStyled> for Lisp {
    fn from(value: LispStyled) -> Self {
        Lisp::Atomic(Box::new(value))
    }
}

impl LispStyled {
    pub fn new<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            style: PrettyPrintKind::Normal,
        }
    }
    pub fn get_text(&self) -> Cow<'static, str> {
        match &self.text {
            Cow::Borrowed(s) => { Cow::Borrowed(s) }
            Cow::Owned(s) => { Cow::Owned(s.to_owned()) }
        }
    }
    pub fn with_style(self, style: PrettyPrintKind) -> Self {
        Self { style, ..self }
    }
    pub fn as_lisp(&self) -> Lisp {
        Lisp::Atomic(Box::new(self.clone()))
    }
    pub fn as_concat(&self) -> Lisp {
        Lisp::Concat(vec![self.as_lisp()])
    }
    pub fn as_sequence(&self) -> Lisp {
        Lisp::Sequence(vec![self.as_lisp()])
    }
}


impl Lisp {
    pub fn new(capacity: usize) -> Self {
        Self::Sequence(Vec::with_capacity(capacity))
    }

    pub fn symbol<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled {
            text: text.into(),
            style: PrettyPrintKind::Operator,
        }.into()
    }

    pub fn string<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled {
            text: text.into(),
            style: PrettyPrintKind::String,
        }.into()
    }
    pub fn number<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled {
            text: text.into(),
            style: PrettyPrintKind::Number,
        }.into()
    }
    pub fn unit<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled {
            text: text.into(),
            style: PrettyPrintKind::Annotation,
        }.into()
    }

    pub fn keyword<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled {
            text: text.into(),
            style: PrettyPrintKind::Keyword,
        }.into()
    }
    pub fn operator<S, I, T>(text: S, items: I) -> Self where S: Into<Cow<'static, str>>, I: IntoIterator<Item=T>, T: Into<Lisp> {
        let mut terms = vec![];
        terms.push(LispStyled {
            text: text.into(),
            style: PrettyPrintKind::Operator,
        }.into());
        for item in items {
            terms.push(item.into());
        }
        Lisp::Sequence(terms)
    }
}