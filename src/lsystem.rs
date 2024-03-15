use std::hash::Hash;

use crate::rules::Rules;

#[derive(Default)]
pub struct LSystem<T: Clone + Hash + Eq> {
    rules: Rules<T>,
    axiom: Vec<T>,
    state: Vec<T>,
}

impl<T> LSystem<T>
where
    T: Clone + Hash + Eq,
{
    pub fn new(rules: Rules<T>, axiom: &[T]) -> Self {
        let v = axiom.to_vec();

        LSystem {
            rules,
            axiom: v.clone(),
            state: v.clone(),
        }
    }

    pub fn state(&self) -> &Vec<T> {
        &self.state
    }

    pub fn reset(&mut self) {
        self.state = self.axiom.clone()
    }
}

impl<T> Iterator for LSystem<T>
where
    T: Clone + Hash + Eq,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let mut next_state = Vec::with_capacity(self.state.len());
        let mut changed = false;

        for k in &self.state {
            if let Some(v) = self.rules.map(k) {
                next_state.extend(v);
                changed = true;
            } else {
                next_state.push(k.clone());
            }
        }

        if changed {
            self.state = next_state;
            Some(self.state.clone())
        } else {
            None
        }
    }
}
