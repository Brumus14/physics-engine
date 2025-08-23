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

        if let Some(additional) = id.checked_sub(self.items.len()) {
            self.items.reserve(additional);
        }

        self.items[id] = Some(item);
        id
    }

    pub fn remove(&mut self, id: Id) {
        if self.id_pool.free(id) {
            self.items.pop();
        }

        if let Some(item) = self.items.get_mut(id) {
            *item = None;
        }
    }
}
