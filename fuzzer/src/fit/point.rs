use core::fmt;

use crate::{mut_input::MutInput};
use serde_derive::{Deserialize, Serialize};
use std::{hash::{Hasher, Hash}};
#[derive(Clone, Deserialize, Serialize)]
pub struct Point {
    pub inputs: Vec<u64>,
    pub output: u64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        if self.inputs.len() != other.inputs.len() {
            return false
        }
        for i in 0..self.inputs.len() {
            if self.inputs[i] != other.inputs[i] {
                return false
            }
        }
        true
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in &self.inputs {
            i.hash(state);
        }
    }
}

impl Point {
    pub fn from(input: &MutInput, output: u64) -> Self {
        let mut inputs = vec![];
        for i in 0..input.len() {
            let value = input.get_entry(i);
            inputs.push(value);
        }
        Self { inputs, output }
    }
}



impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(input: ")?;
        for i in &self.inputs {
            write!(f, "{}, ", i)?
        }
        write!(f, "output: {})\n", self.output)
    }
}

