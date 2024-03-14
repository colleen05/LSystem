pub trait Rules<T> {
    fn map(&self, symbol: T) -> Option<Vec<T>>;
}
