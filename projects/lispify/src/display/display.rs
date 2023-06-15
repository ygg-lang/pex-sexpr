use super::*;

impl PrettyPrint for Lisp {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Lisp::Atomic(value) => value.pretty(theme),
            Lisp::Concat(values) => {
                let mut terms = PrettySequence::new(values.len());
                for value in values {
                    terms += value.pretty(theme);
                }
                terms.into()
            }
            Lisp::Sequence(values) => {
                let mut terms = PrettySequence::new(values.len() * 2 + 1);
                terms += "(";
                terms += theme.join(values.iter().cloned(), PrettyTree::line_or_space()).nest(2).group();
                terms += ")";
                terms.into()
            }
        }
    }
}

impl PrettyPrint for LispStyled {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.style {
            PrettyPrintKind::Normal => theme.text(self.get_text()),
            PrettyPrintKind::Keyword => theme.keyword(self.get_text()),
            PrettyPrintKind::String => theme.string(self.get_text()),
            PrettyPrintKind::Number => theme.number(self.get_text()),
            PrettyPrintKind::Annotation => theme.annotation(self.get_text()),
            PrettyPrintKind::Argument => theme.argument(self.get_text(), false),
            PrettyPrintKind::ArgumentMutable => theme.argument(self.get_text(), true),
            PrettyPrintKind::Local => theme.variable(self.get_text(), false),
            PrettyPrintKind::LocalMutable => theme.variable(self.get_text(), true),
            PrettyPrintKind::Operator => theme.operator(self.get_text()),
            PrettyPrintKind::Structure => theme.structure(self.get_text()),
            PrettyPrintKind::Class => theme.structure(self.get_text()),
            PrettyPrintKind::Union => theme.structure(self.get_text()),
            PrettyPrintKind::UnionDisjoint => theme.structure(self.get_text()),
            PrettyPrintKind::Variant => theme.argument(self.get_text(), false),
            PrettyPrintKind::Interface => theme.interface(self.get_text()),
            PrettyPrintKind::Trait => theme.interface(self.get_text()),
        }
    }
}

impl Display for Lisp {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let theme = PrettyProvider::new(80);
        f.write_str(&self.pretty_string(&theme))
    }
}
