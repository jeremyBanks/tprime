pub trait Grid<IndexType>
where
    IndexType: Copy,
{
    fn neighbours(&self, index: IndexType) -> Vec<IndexType>;

    /// Returns an lower bound on the number of neighbour connections that must be traversed to get from point a to point b.
    fn distance(&self, a: IndexType, b: IndexType) -> u32 {
        0
    }
}
