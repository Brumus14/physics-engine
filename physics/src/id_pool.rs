use std::collections::HashSet;

pub type Id = usize;

pub struct IdPool {
    next_id: Id,
    free_ids: HashSet<Id>,
    max_delta: Id,
}

impl IdPool {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            free_ids: HashSet::new(),
            max_delta: 0,
        }
    }

    pub fn next(&mut self) -> Id {
        if let Some(&id) = self.free_ids.iter().next() {
            self.free_ids.take(&id).unwrap()
        } else {
            // Grow pool
            self.next_id += 1;
            self.next_id - 1
        }
    }

    pub fn free(&mut self, id: Id) {
        if let Some(max) = self.max() {
            if id > max {
                return;
            }

            if id == max {
                // Shrink pool
                self.next_id -= 1;

                // Remove trailing freed ids
                while let Some(new_max) = self.max() {
                    if !self.free_ids.contains(&new_max) {
                        break;
                    }

                    self.free_ids.take(&new_max);
                    self.next_id -= 1;
                }
            } else if id < max {
                self.free_ids.insert(id);
            }
        }
    }

    pub fn max(&self) -> Option<Id> {
        self.next_id.checked_sub(1)
    }
}
