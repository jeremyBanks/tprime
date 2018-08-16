use std::marker::PhantomData;

/// A fixed square grid allowing diagonal movement.
pub struct ASquareGrid<Axis>
where
    Axis: Into<usize>,
{
    _axis: PhantomData<Axis>,
}

pub type SquareGrid = ASquareGrid<u32>;
