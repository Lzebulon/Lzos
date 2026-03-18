use core::panicking::panic;

use crate::{
    printkln,
    proc::{self, switch_context_proc, Proc},
};

/// Trait Schelduler permit to implement different
/// Schelduler and to test them.
trait Schelduler {
    type ScheldulerItems;

    /// run the next program
    fn next(&mut self);

    /// add a new process
    fn add(&mut self, proc: Self::ScheldulerItems);

    /// remove a process
    fn remove(&mut self, proc: Self::ScheldulerItems);
}

// ```rust
// const sch = Schelduler::new();
//
// fn timer_interrupt() {
//     sch.next().run();
// }
// ```

struct DefaultSchelduler {
    current_proc: usize,
    processes: [Option<Proc>; proc::MAX_PROCS],
}

impl Schelduler for DefaultSchelduler {
    type ScheldulerItems = Proc;

    fn next(&mut self) {
        let last_proc = self.current_proc;

        for i in last_proc + 1..proc::MAX_PROCS {
            if self.processes[i].is_some() {
                self.current_proc = i;

                switch_context_proc(
                    self.processes[last_proc].unwrap(),
                    self.processes[i].unwrap(),
                );

                return;
            }
        }

        for i in 0..last_proc {
            if self.processes[i].is_some() {
                self.current_proc = i;
                switch_context_proc(
                    self.processes[last_proc].unwrap(),
                    self.processes[i].unwrap(),
                );
                return;
            }
        }

        // there is only one process so we don't need to switch
        return;
    }

    fn add(&mut self, proc: Self::ScheldulerItems) {
        for mut e in self.processes {
            if e.is_none() {
                e.replace(proc);
                return;
            }
        }

        panic!("It's not possible to add a process");
    }

    fn remove(&mut self, proc: Self::ScheldulerItems) {
        printkln!("for the moment we can't remove an proc");
    }
}
