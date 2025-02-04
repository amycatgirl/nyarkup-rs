use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NoChildren;

impl fmt::Display for NoChildren {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This node does not have any children!")
    }
}

impl fmt::Debug for NoChildren {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
