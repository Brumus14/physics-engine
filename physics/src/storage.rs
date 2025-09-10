use std::collections::{BinaryHeap, HashSet, binary_heap};

use crate::id_pool::{Id, IdPool};

pub struct Storage<T> {
    id_pool: IdPool,
    items: Vec<Option<T>>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self {
            id_pool: IdPool::new(),
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: T) -> Id {
        let id = self.id_pool.next();

        // Pool has grown
        if id == self.id_pool.max() {
            self.items.push(Some(item));
        } else {
            self.items[id] = Some(item);
        }

        id
    }

    pub fn remove(&mut self, id: Id) {
        if id > self.id_pool.max() {
            return;
        }

        // Pool has shrunk
        if id == self.id_pool.max() {
            self.items.pop();

            // Remove trailing empty items
            while matches!(self.items.last(), Some(None)) {
                self.items.pop();
            }
        } else {
            self.items[id] = None;
        }

        self.id_pool.free(id);
    }
}
