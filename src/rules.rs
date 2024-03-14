use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
pub struct Rules<T: Hash + Eq> {
    pub productions: HashMap<T, Vec<T>>,
}

impl<T> Rules<T>
where
    T: Clone + Hash + Eq,
{
    pub fn new() -> Self {
        Rules {
            productions: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: T, v: Vec<T>) -> Option<Vec<T>> {
        self.productions.insert(k, v)
    }

    pub fn remove(&mut self, k: &T) -> Option<Vec<T>> {
        self.productions.remove(k)
    }

    pub fn contains(&self, k: &T) -> bool {
        self.productions.contains_key(k)
    }

    pub fn map(&self, k: &T) -> Option<Vec<T>> {
        self.productions.get(k).cloned()
    }
}
