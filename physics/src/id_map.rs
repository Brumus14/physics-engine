use std::collections::HashSet;

pub type Id = usize;

#[derive(Debug)]
pub struct IdMap<T> {
    values: Vec<Option<T>>,
    next_id: Id,
    // Maybe use vecdequeue and if free id isnt free anymore just use next
    free_ids: HashSet<Id>,
}

impl<T> IdMap<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            next_id: 0,
            free_ids: HashSet::new(),
        }
    }

    pub fn add(&mut self, value: T) -> Id {
        if let Some(&id) = self.free_ids.iter().next() {
            // Use a free space
            self.values[id] = Some(value);
            self.free_ids.take(&id).unwrap()
        } else {
            // Grow size
            self.next_id += 1;
            self.values.push(Some(value));
            self.next_id - 1
        }
    }

    pub fn remove(&mut self, id: Id) {
        if id >= self.next_id || self.values.is_empty() {
            return;
        }

        if id == self.next_id - 1 {
            self.values[id] = None;

            while let Some(None) = self.values.last() {
                self.values.pop();
                self.free_ids.remove(&(self.next_id - 1));
                self.next_id -= 1;

                if self.next_id == 0 {
                    break;
                }
            }
        } else {
            if self.values[id].is_some() {
                self.values[id] = None;
                self.free_ids.insert(id);
            }
        }
    }

    pub fn clear(&mut self) {
        self.values = Vec::new();
        self.next_id = 0;
        self.free_ids = HashSet::new();
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        if let Some(value) = self.values.get(id) {
            value.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        if let Some(value) = self.values.get_mut(id) {
            value.as_mut()
        } else {
            None
        }
    }

    pub fn get_disjoint_mut<const N: usize>(&mut self, ids: [Id; N]) -> [&mut Option<T>; N] {
        self.values.get_disjoint_mut(ids).unwrap()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.values.iter().filter_map(|v| v.as_ref())
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut().filter_map(|v| v.as_mut())
    }
}
