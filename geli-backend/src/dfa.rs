#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NodeBody<T> {
    Incomplete,
    Finished(T),
}
