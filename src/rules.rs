use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::Hash,
};

#[derive(Default, Clone)]
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

impl<T> Display for Rules<T>
where
    T: Hash + Eq + ToString,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.productions
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{} -> {}",
                        k.to_string(),
                        v.iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
