pub trait LRules<T> {
    fn map(&self, symbol: T) -> Option<Vec<T>>;
}
