use angora_common::shm;

use crate::cond_stmt::CondStmt;

use super::point::Point;


pub struct Model {
    cmpid: u32,

    // f(X) = theta0 + theta1 * x1 + ... + thetan * xn
    var_num: usize,
    thetas: Vec<i64>,
}

impl Model {
    pub fn new(cond: &CondStmt) -> Self {
        Self {
            cmpid: cond.base.cmpid,
            var_num: cond.var_num,
            thetas: vec![],
        }
    }

    pub fn fitting(&self) {

    }

    pub fn loss_value() {

    }
}