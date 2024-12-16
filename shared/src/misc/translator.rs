use fxhash::FxHashMap as HashMap;
use std::{cmp::Eq, hash::Hash};

pub struct Translator<T>
where
    T: Hash,
    T: Eq,
{
    pub map: HashMap<T, usize>,
    next_id: usize,
}

impl<T> Translator<T>
where
    T: Hash,
    T: Eq,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
            next_id: 0,
        }
    }

    pub fn translate(&mut self, value: T) -> usize {
        if let Some(id) = self.map.get(&value) {
            return *id;
        }

        let id = self.next_id;
        self.map.insert(value, id);
        self.next_id += 1;

        id
    }
}
