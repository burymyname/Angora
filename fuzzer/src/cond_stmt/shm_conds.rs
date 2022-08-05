use super::CondOutput;
use crate::cond_stmt::{self, output::translate_unsign_to_sign};
use crate::mut_input::MutInput;
use crate::fit::point::Point;
use angora_common::{cond_stmt_base::CondStmtBase, defs, shm};
use std;

pub struct ShmConds {
    pub cond: shm::SHM<CondStmtBase>,
}

impl ShmConds {
    pub fn new() -> Self {
        Self {
            cond: shm::SHM::<CondStmtBase>::new(),
        }
    }

    #[inline(always)]
    pub fn get_id(&self) -> i32 {
        self.cond.get_id()
    }

    #[inline(always)]
    fn get_len(&self) -> usize {
        self.cond.level as usize
    }

    #[inline(always)]
    fn set_len(&mut self, len: usize) {
        self.cond.level = len as u32;
    }

    #[inline(always)]
    fn reset_reachable_state(&mut self) {
        self.cond.lb1 = std::u32::MAX;
    }

    #[inline(always)]
    pub fn is_cond_reachable(&self) -> bool {
        self.cond.lb1 < std::u32::MAX
    }

    pub fn set(&mut self, cond: &cond_stmt::CondStmt) -> bool {
        if self.get_len() == 0 {
            *self.cond = cond.base.clone();
            self.set_len(1);
            self.reset_reachable_state();
            true
        } else {
            self.reset_reachable_state();
            false
        }
    }

    pub fn clear(&mut self) {
        self.cond.cmpid = 0;
        self.cond.order = 0;
        self.cond.context = 0;
        self.set_len(0);
    }

    pub fn get_cond_output(&self) -> u64 {
        if !self.is_cond_reachable() {
            debug!("unreachable, output is MAX");
            return defs::UNREACHABLE;
        }
        let mut output = self.cond.get_output();
        if output == defs::UNREACHABLE {
            output -= 1;
        }
        output
    }

    pub fn get_cond_point(&self, input: &MutInput, to_arg: u8) -> Option<Point> {
        debug!("input={:?}", input);
        if !self.is_cond_reachable() || self.cond.is_ptr() {
            return None;
        }

        let a = self.cond.arg1;
        let b = self.cond.arg2;

        let val = match to_arg {
            1 => a,
            2 => b,
            _ => {return None;},
        };
        
        let mut i = 0_i64;
        if self.cond.is_signed() {
            i = translate_unsign_to_sign(val, self.cond.size);
        }
        debug!("point: input={:?}, output={}/{}", input, val, i);
        let point = Point::from(input, val);
        Some(point)
    }
}
