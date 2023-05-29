use std::ops::Range;
use std::str::FromStr;
use pex::{ParseResult, ParseState};
use pex::helpers::decimal_string;

#[derive(Clone, Debug)]
pub enum SExpr {
    Number(SNumber),
    List(Box<SList>),
    Symbol(String),
    String(String),
}

#[derive(Clone, Debug)]
pub struct SList {
    value: Vec<SExpr>,
    range: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct SNumber {
    value: f64,
    range: Range<usize>,
}

impl SExpr {
    pub fn parse(&self, state: ParseState) -> ParseResult<()> {
        todo!()
    }
}


impl SNumber {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, s) = decimal_string(input)?;
        let value = f64::from_str(s)?;
        let range = state.away_from(input);
        state.finish(Self {
            value,
            range,
        })
    }
}