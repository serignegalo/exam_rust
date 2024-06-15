use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Self { drops: Cell::new(0) , states: RefCell::new(Vec::new()) }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        let new_thread = Thread::new_thread(pid, c, self);
        (pid, new_thread)
    }
    pub fn track_worker(&self) -> usize {
        let mut state = self.states.borrow_mut();
        let pid = state.len();
        state.push(false);
        pid
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        let mut state = self.states.borrow_mut();

        if state[id] {
            panic!("{} is already dropped", id);
        } else {
            state[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Self { pid: p, cmd: c, parent: t}
    }
    pub fn skill(self) {}
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid)
    }
}


