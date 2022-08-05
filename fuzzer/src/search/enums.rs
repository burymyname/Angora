/*
Variables value enumeration search
 */

use super::*;

pub struct EnumSearch<'a> {
    handler: SearchHandler<'a>,
}

impl<'a> EnumSearch<'a> {
    pub fn new(handler: SearchHandler<'a>) -> Self {
        Self{
            handler,
        }
    }

    // return step and unsigned int max value
    fn get_step_and_max(&self, size: usize) -> (u64, u64) {
        match size {
            1 => (1, std::u8::MAX as u64),
            2 => (std::u16::MAX  as u64 / config::MAX_ENUM_FIT_POINTS_NUM, std::u16::MAX as u64),
            4 => (std::u32::MAX as u64 / config::MAX_ENUM_FIT_POINTS_NUM, std::u32::MAX as u64),
            8 => (std::u64::MAX as u64 / config::MAX_ENUM_FIT_POINTS_NUM, std::u64::MAX),
            _ => (1, std::u8::MAX as u64),
        }
    }


    pub fn run(&mut self) {
        self.handler.search = SearchMethod::Em;

        let mut input = self.handler.get_f_input();

        for i in 0..input.len() {
            let size = input.get_entry_len(i);
            let (step, max) = self.get_step_and_max(size);
            let mut n = 0;
            while n < max {
                let val = n;
                input.set(i, val);
                self.handler.execute_cond(&input);
                n += step;
            }
        }

        let min_fit_point = config::MAX_ENUM_FIT_POINTS_NUM as usize;
        if self.handler.cond.point1_num() >=  min_fit_point
            || self.handler.cond.point2_num() >= min_fit_point {
            return;
        }

        input = self.handler.get_f_input();
        let orig_input_val = input.get_value();
        loop {
            if self.handler.is_stopped_or_skip() || self.handler.cond.point1_num() >= min_fit_point 
                || self.handler.cond.point2_num() >= min_fit_point {
                break;
            }

            input.assign(&orig_input_val);
            input.randomize_all();
            self.handler.execute_cond(&input);
        }
    }
}