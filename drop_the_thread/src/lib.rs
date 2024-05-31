use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        self.states.borrow_mut().push(false);
        (self.track_worker(), Thread {
            pid: self.track_worker(),
            cmd: c,
            parent: &self
        })
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len() - 1
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap()
    }
    pub fn add_drop(&self, id: usize) {
        let mut temp = self.states.borrow_mut();
        if let Some(el) = temp.get_mut(id) { 
            if *el == true {
                panic!("{} is already dropped", id)
            } else {
                *el = true;
                self.drops.set(self.drops.get() + 1);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid: p,
            cmd: c,
            parent: t
        } 
    }
    pub fn skill(self) {

    }
}
impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    } 
}