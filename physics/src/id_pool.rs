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

    pub fn free(&mut self, id: Id) {
        if id == self.next_id - 1 {
            self.next_id -= 1;
        } else {
            self.free_ids.push(id);
        }
    }

    pub fn max(&self) -> Id {
        self.next_id - 1
    }
}
