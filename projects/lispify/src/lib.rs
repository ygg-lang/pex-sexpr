#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod display;
mod errors;
pub mod helpers;
// mod parser;
mod traits;
pub use self::{
    display::{Lisp, LispStyled},
    traits::Lispify,
};
pub use crate::errors::{Error, Result};
pub use pretty_print::PrettyPrint;
