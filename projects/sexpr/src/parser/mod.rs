use std::ops::Range;
use std::str::FromStr;
use pex::{ParseResult, ParseState};
use pex::helpers::decimal_string;

pub struct SExpr {}

#[derive(Clone, Debug)]
pub struct Number {
    value: f64,
    range: Range<usize>,
}


impl SExpr {
    pub fn parse(&self, state: ParseState) -> ParseResult<()> {}
}


impl Number {
    pub fn parse(&self, input: ParseState) -> ParseResult<Self> {
        let (state, s) = decimal_string(input)?;
        let value = f64::from_str(s)?;
        let range = state.away_from(input);
        state.finish(Self {
            value,
            range,
        })
    }
}