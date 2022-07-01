use core::fmt;

use crate::mut_input::MutInput;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Point {
    input: MutInput,
    output: u64,
}

impl Point {
    pub fn new(
        input: &MutInput,
        output: u64,
    ) -> Self {
        Self {
            input: input.clone(),
            output,
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(input: {:?} output: {})", self.input, self.output)
    }
}

