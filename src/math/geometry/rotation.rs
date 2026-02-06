use crate::math::geometry::{positioning::Position, EmptyShape, Shape};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// A shape with a 1 dimensional position and an x dimensional rotation
///
/// THERE IS NO FUCKING ROTATION, VELOCITY DESCRIBES DIRECTION
pub struct Rotation1D<V, S: Shape<V>, P: Position<V, S>> {
    /// The position
    pub position: P,
    #[allow(missing_docs)]
    pub _value_type: core::marker::PhantomData<V>,
    #[allow(missing_docs)]
    pub _shape: core::marker::PhantomData<S>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// A shape with a 2 dimensional position and an x dimensional rotation
pub struct Rotation2D<
    V,
    S: Shape<V>,
    P: Position<V, S>,
    A: Position<V, EmptyShape>,
> {
    /// The position
    pub position: P,
    /// The rotation
    pub rotation: V,
    /// The pivot which to rotate around
    pub pivot: A,
    /// After storing the rotation, this is precomputed for further usage
    pub precomputed: (V, V),
    #[allow(missing_docs)]
    pub _shape: core::marker::PhantomData<S>,
}
