use super::CondState;
use crate::fit::{point::Point, point_set::PointSet};
use crate::{fuzz_type::FuzzType};
use angora_common::{cond_stmt_base::CondStmtBase, defs, tag::TagSeg};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::{
    hash::{Hash, Hasher},
    time::Duration,
    path::Path,
    io::Write,
};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CondStmt {
    pub base: CondStmtBase,
    pub offsets: Vec<TagSeg>,
    pub offsets_opt: Vec<TagSeg>,
    pub variables: Vec<u8>,
    
    pub var_num: usize,
    pub var_num_opt: usize,
    pub offsets_to_arg: u8,
    pub offsets_opt_to_arg: u8,

    pub speed: u32,
    pub is_desirable: bool, // non-convex
    pub is_consistent: bool, // track and fast program cmpid should be consistent
    pub fuzz_times: usize,
    pub state: CondState,
    pub num_minimal_optima: usize,
    pub linear: bool,
    pub fuzz_duration: Duration,

    pub points1: PointSet,
    pub points2: PointSet,
}

impl PartialEq for CondStmt {
    fn eq(&self, other: &CondStmt) -> bool {
        self.base == other.base
    }
}

impl Eq for CondStmt {}

impl Hash for CondStmt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.cmpid.hash(state);
        self.base.context.hash(state);
        self.base.order.hash(state);
    }
}

impl CondStmt {
    pub fn new() -> Self {
        let cond_base = Default::default();
        Self {
            base: cond_base,
            offsets: vec![],
            offsets_opt: vec![],
            offsets_to_arg: 0,
            offsets_opt_to_arg: 0,
            variables: vec![],
            var_num: 0,
            var_num_opt: 0,
            speed: 0,
            is_consistent: true,
            is_desirable: true,
            fuzz_times: 0,
            state: CondState::default(),
            num_minimal_optima: 0,
            linear: false,
            fuzz_duration: Duration::ZERO,
            points1: PointSet::default(),
            points2: PointSet::default(),
        }
    }

    pub fn from(cond_base: CondStmtBase) -> Self {
        let mut cond = Self::new();
        cond.base = cond_base;
        cond.points1.set_sign(cond.base.is_signed());
        cond.points1.set_size(cond.base.size);
        cond.points2.set_sign(cond.base.is_signed());
        cond.points2.set_size(cond.base.size);
        cond
    }

    pub fn get_fuzz_type(&self) -> FuzzType {
        match self.base.op {
            defs::COND_AFL_OP => FuzzType::AFLFuzz,
            defs::COND_LEN_OP => FuzzType::LenFuzz,
            defs::COND_FN_OP => FuzzType::CmpFnFuzz,
            _ => {
                if self.base.is_explore() {
                    FuzzType::ExploreFuzz
                } else if self.base.is_exploitable() {
                    FuzzType::ExploitFuzz
                } else {
                    FuzzType::OtherFuzz
                }
            },
        }
    }

    pub fn is_tainted(&self) -> bool {
        self.offsets.len() > 0
    }

    pub fn is_bool(&self) -> bool {
        (self.base.may_be_bool() && !self.is_desirable) || (self.base.op & defs::COND_BOOL_MASK) > 0
    }

    pub fn mark_as_done(&mut self) {
        self.base.condition = defs::COND_DONE_ST;
        // self.clear();
    }

    pub fn clear(&mut self) {
        self.offsets = vec![];
        self.offsets_opt = vec![];
        self.variables = vec![];
    }

    pub fn is_discarded(&self) -> bool {
        self.is_done() || self.state.is_unsolvable() || self.state.is_timeout()
    }

    pub fn is_first_time(&self) -> bool {
        self.fuzz_times == 1
    }

    pub fn get_afl_cond(id: usize, speed: u32, edge_num: usize) -> Self {
        let mut afl_cond = Self::new();
        afl_cond.speed = speed;
        afl_cond.base.op = defs::COND_AFL_OP;
        afl_cond.base.cmpid = id as u32;
        afl_cond.base.context = 0;
        afl_cond.base.order = 0;
        afl_cond.base.arg1 = edge_num as u64;
        afl_cond
    }

    pub fn is_done(&self) -> bool {
        self.base.is_done()
    }

    pub fn add_duration(&mut self, time: Duration) {
        self.fuzz_duration += time;
    }

    pub fn insert_point(&mut self, point: &Point) {
        match self.state {
            CondState::Offset | CondState::OneByte => {
                if !self.points1.insert(point.clone()) {
                    debug!("insert points failed!");
                }
            },
            CondState::OffsetOpt => {
                if !self.points2.insert(point.clone()) {
                    debug!("insert points failed!");
                }
            },
            _ => {},
        }

    }

    pub fn point1_num(&self) -> usize {
        self.points1.len()
    }

    pub fn point2_num(&self) -> usize {
        self.points2.len()
    }

    pub fn dump_points(&self, dir: &Path) {
        let point_file_name = format!("{}_{}_{}", self.base.cmpid, self.base.context, self.base.order);
        let file_path = dir.join(point_file_name);
        if file_path.exists() {
            debug!("point file exists!");
            fs::remove_file(file_path.as_path()).unwrap();
        }

        let mut f = fs::File::create(file_path.as_path()).expect("Could not save point file");
        let target = match self.offsets_to_arg {
            2 => self.base.arg1,
            _ => self.base.arg2,        
        };
        writeln!(f, "cmpid: {}, context: {}, order: {}, target: {}", self.base.cmpid, self.base.context, self.base.order, target).unwrap();
        writeln!(f, "{:?}", self.points1).unwrap();

        // TODO: dump points2
    }

    pub fn has_points(&self) -> bool {
        self.point1_num() > 0
    }

}
