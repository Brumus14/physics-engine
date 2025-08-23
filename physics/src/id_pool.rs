pub type Id = usize;

pub struct IdPool {
    next_id: Id,
    free_ids: Vec<Id>,
}

impl IdPool {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            free_ids: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Id {
        if let Some(id) = self.free_ids.pop() {
            id
        } else {
            self.next_id += 1;
            self.next_id - 1
        }
    }

    // Returns true if max decreased
    pub fn free(&mut self, id: Id) -> bool {
        if id == self.max() {
            self.next_id -= 1;
            return true;
        } else if id < self.max() {
            self.free_ids.push(id);
        }

        false
    }

    pub fn max(&self) -> Id {
        self.next_id - 1
    }
}
