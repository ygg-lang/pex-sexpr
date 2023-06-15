use super::*;

impl From<LispStyled> for Lisp {
    fn from(value: LispStyled) -> Self {
        Lisp::Atomic(Box::new(value))
    }
}

impl LispStyled {
    /// Create a new lisp styled value
    pub fn new<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Self { text: text.into(), style: PrettyPrintKind::Normal }
    }
    /// Get the text of the lisp styled value
    pub fn get_text(&self) -> Cow<'static, str> {
        match &self.text {
            Cow::Borrowed(s) => Cow::Borrowed(s),
            Cow::Owned(s) => Cow::Owned(s.to_owned()),
        }
    }
    /// Get the style of the lisp styled value
    pub fn with_style(self, style: PrettyPrintKind) -> Self {
        Self { style, ..self }
    }
}

impl Lisp {
    /// Create a new lisp sequence
    pub fn new(capacity: usize) -> Self {
        Self::Sequence(VecDeque::with_capacity(capacity))
    }
    /// Create a new lisp sequence
    pub fn extend<I, T>(&mut self, items: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<Lisp>,
    {
        for item in items {
            *self += item.into();
        }
    }
    /// Create a new lisp sequence
    pub fn extend_slice<T>(&mut self, items: &[T])
    where
        T: Into<Lisp> + Clone,
    {
        for item in items.iter().cloned() {
            *self += item.into();
        }
    }

    /// Create a new lisp sequence
    pub fn symbol<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled { text: text.into(), style: PrettyPrintKind::Operator }.into()
    }
    /// Create a new lisp sequence
    pub fn string<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled { text: text.into(), style: PrettyPrintKind::String }.into()
    }
    /// Create a new lisp sequence
    pub fn number<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled { text: text.into(), style: PrettyPrintKind::Number }.into()
    }
    /// Create a new lisp sequence
    pub fn unit<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled { text: text.into(), style: PrettyPrintKind::Annotation }.into()
    }
    /// Create a new lisp sequence
    pub fn keyword<S: Into<Cow<'static, str>>>(text: S) -> Self {
        LispStyled { text: text.into(), style: PrettyPrintKind::Keyword }.into()
    }
    /// Create a new lisp sequence
    pub fn operator<S, I, T>(text: S, items: I) -> Self
    where
        S: Into<Cow<'static, str>>,
        I: IntoIterator<Item = T>,
        T: Into<Lisp>,
    {
        let mut terms = VecDeque::new();
        terms.push_back(LispStyled { text: text.into(), style: PrettyPrintKind::Operator }.into());
        for item in items {
            terms.push_back(item.into());
        }
        Lisp::Sequence(terms)
    }
}
