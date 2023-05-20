use pretty::RcDoc;
use pretty::Doc;

///
#[derive(Debug)]
pub enum Lisp {
    List(Vec<Lisp>),
    Atom(u32),
}

impl Lisp {
    /// Return a pretty printed format of self.
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            Lisp::Atom(ref x) => RcDoc::as_string(x),
            Lisp::List(ref xs) =>
                RcDoc::text("(")
                    .append(RcDoc::intersperse(xs.into_iter().map(|x| x.to_doc()), Doc::line()).nest(1).group())
                    .append(RcDoc::text(")"))
        }
    }
}

impl Lisp {
    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

#[test]
fn test() {
    let atom = Lisp::Atom(5);
    assert_eq!("5", atom.to_pretty(10));
    let list = Lisp::List(vec![Lisp::Atom(1), Lisp::Atom(2), Lisp::Atom(3)]);
    assert_eq!("(1 2 3)", list.to_pretty(10));
    assert_eq!("\
(1
 2
 3)", list.to_pretty(5));
}