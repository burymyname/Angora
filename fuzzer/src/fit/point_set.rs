use core::fmt;

use crate::cond_stmt::output::translate_unsign_to_sign;

use super::point::Point;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashSet, fs, fmt::write};

#[derive(Clone, Deserialize, Serialize)]
pub struct PointSet {
    sign: bool,
    size: u32,
    pub points: HashSet<Point>,
}

impl Default for PointSet {
    fn default() -> Self {
        Self { sign: false, size: 0, points: HashSet::new() }
    }
}

impl PointSet {

    pub fn new(sign: bool, size: u32) -> Self {
        Self { sign, size, points: HashSet::new() }
    }

    pub fn set_sign(&mut self, sign: bool) {
        self.sign = sign;
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn insert(&mut self, p: Point) -> bool {
        self.points.insert(p)
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

}

impl fmt::Debug for PointSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.points {
            write!(f, "{:?}", p)?
        }    
        Ok(())
    }
}
