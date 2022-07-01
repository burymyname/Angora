use super::*;
use angora_common::defs;
use std::{fs, io::prelude::*};

impl Drop for Depot {
    fn drop(&mut self) {
        info!("dump constraints and chart..");
        let dir = self.dirs.inputs_dir.parent().unwrap();

        let mut log_q = fs::File::create(dir.join(defs::COND_QUEUE_FILE)).unwrap();
        writeln!(
            log_q,
            "cmpid, context, order, belong, p, op, condition, arg1, arg2, is_desirable, offsets, state, fuzz_times, fuzz_duration, var_num, points"
        )
        .unwrap();

        let mut log_p = fs::File::create(dir.join(defs::COND_POINT_FILE)).unwrap();

        let q = self.queue.lock().unwrap();

        for (cond, p) in q.iter() {
            if !cond.base.is_afl() {
                let mut offsets = vec![];
                for off in &cond.offsets {
                    offsets.push(format!("{}-{}", off.begin, off.end));
                }

                writeln!(
                    log_q,
                    "{}, {}, {}, {}, {}, {}, {}, {:x}, {:x}, {}, {}, {:?}, {}, {:?}, {}",
                    cond.base.cmpid,
                    cond.base.context,
                    cond.base.order,
                    cond.base.belong,
                    p,
                    cond.base.op,
                    cond.base.condition,
                    cond.base.arg1,
                    cond.base.arg2,
                    cond.is_desirable,
                    offsets.join("&"),
                    cond.state,
                    cond.fuzz_times,
                    cond.fuzz_duration,
                    cond.var_num,
                )
                .unwrap();

                writeln!(
                    log_p,
                    "{}, {}, {:?}",
                    cond.base.cmpid,
                    cond.var_num,
                    cond.points,
                ).unwrap();
            }
        }

        

    }
}
