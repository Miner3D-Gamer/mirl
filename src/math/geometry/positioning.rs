use crate::math::geometry::Shape;

/// Trait for types that have an X coordinate accessor.
pub const trait GetPosX<T> {
    /// Returns a mutable reference to the X coordinate.
    fn get_x_mut(&mut self) -> &mut T;
    /// Returns a copy of the X coordinate.
    fn get_x(&self) -> T;
    /// Sets the X coordinate to a new value.
    fn set_x(&mut self, value: T);
}

/// Trait for types that have a Y coordinate accessor.
pub const trait GetPosY<T> {
    /// Returns a mutable reference to the Y coordinate.
    fn get_y_mut(&mut self) -> &mut T;
    /// Returns a copy of the Y coordinate.
    fn get_y(&self) -> T;
    /// Sets the Y coordinate to a new value.
    fn set_y(&mut self, value: T);
}

/// Trait for types that have a Z coordinate accessor.
pub const trait GetPosZ<T> {
    /// Returns a mutable reference to the Z coordinate.
    fn get_z_mut(&mut self) -> &mut T;
    /// Returns a copy of the Z coordinate.
    fn get_z(&self) -> T;
    /// Sets the Z coordinate to a new value.
    fn set_z(&mut self, value: T);
}

/// Trait for types that have a W coordinate accessor.
pub const trait GetPosW<T> {
    /// Returns a mutable reference to the W coordinate.
    fn get_w_mut(&mut self) -> &mut T;
    /// Returns a copy of the W coordinate.
    fn get_w(&self) -> T;
    /// Sets the W coordinate to a new value.
    fn set_w(&mut self, value: T);
}

/// Trait for types that have an associated shape.
pub const trait Position<V, S: Shape<V>> {
    /// Returns a reference to the shape.
    fn get_shape(&self) -> &S;
    /// Returns a mutable reference to the shape.
    fn get_shape_mut(&mut self) -> &mut S;
}

/// A one-dimensional position with an associated shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos1D<V, S: Shape<V>> {
    /// The position along the x coordinate
    pub pos: V,

    /// The given shape
    pub shape: S,
}

impl<V, S: Shape<V>> Pos1D<V, S> {
    #[must_use]
    /// Creates a new 1D position from a coordinate and shape.
    pub const fn new(x: V, shape: S) -> Self {
        Self {
            pos: x,
            shape,
        }
    }
}
impl<V: Copy, S: Shape<V>> Pos1D<V, S> {
    #[must_use]
    /// Get the current pos
    pub const fn get_pos(&self) -> V {
        self.pos
    }
}

impl<V: Copy, S: Shape<V>> const GetPosX<V> for Pos1D<V, S> {
    fn get_x_mut(&mut self) -> &mut V {
        &mut self.pos
    }

    fn get_x(&self) -> V {
        self.pos
    }

    fn set_x(&mut self, value: V) {
        self.pos = value;
    }
}

/// A two-dimensional position with an associated shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos2D<V, S: Shape<V>> {
    /// The position along the x and y coordinate
    pub pos: (V, V),

    /// The given shape
    pub shape: S,
}

impl<V, S: Shape<V>> Pos2D<V, S> {
    #[must_use]
    /// Creates a new 2D position from coordinates and shape.
    pub const fn new(pos: (V, V), shape: S) -> Self {
        Self {
            pos,
            shape,
        }
    }
}

impl<V: Copy, S: Shape<V>> Pos2D<V, S> {
    #[must_use]
    /// Get the current pos
    pub const fn get_pos(&self) -> (V, V) {
        self.pos
    }
}
impl<V: Copy, S: Shape<V>> const GetPosX<V> for Pos2D<V, S> {
    fn get_x_mut(&mut self) -> &mut V {
        &mut self.pos.0
    }

    fn get_x(&self) -> V {
        self.pos.0
    }

    fn set_x(&mut self, value: V) {
        self.pos.0 = value;
    }
}

impl<V: Copy, S: Shape<V>> const GetPosY<V> for Pos2D<V, S> {
    fn get_y_mut(&mut self) -> &mut V {
        &mut self.pos.1
    }

    fn get_y(&self) -> V {
        self.pos.1
    }

    fn set_y(&mut self, value: V) {
        self.pos.1 = value;
    }
}

/// A three-dimensional position with an associated shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos3D<V, S: Shape<V>> {
    /// The position along the x, y, and z coordinate
    pub pos: (V, V, V),

    /// The given shape
    pub shape: S,
}

impl<V, S: Shape<V>> Pos3D<V, S> {
    #[must_use]
    /// Creates a new 3D position from coordinates and shape.
    pub const fn new(pos: (V, V, V), shape: S) -> Self {
        Self {
            pos,
            shape,
        }
    }
}

impl<V: Copy, S: Shape<V>> Pos3D<V, S> {
    #[must_use]
    /// Get the current pos
    pub const fn get_pos(&self) -> (V, V, V) {
        self.pos
    }
}
impl<V: Copy, S: Shape<V>> const GetPosX<V> for Pos3D<V, S> {
    fn get_x_mut(&mut self) -> &mut V {
        &mut self.pos.0
    }

    fn get_x(&self) -> V {
        self.pos.0
    }

    fn set_x(&mut self, value: V) {
        self.pos.0 = value;
    }
}

impl<V: Copy, S: Shape<V>> const GetPosY<V> for Pos3D<V, S> {
    fn get_y_mut(&mut self) -> &mut V {
        &mut self.pos.1
    }

    fn get_y(&self) -> V {
        self.pos.1
    }

    fn set_y(&mut self, value: V) {
        self.pos.1 = value;
    }
}

impl<V: Copy, S: Shape<V>> const GetPosZ<V> for Pos3D<V, S> {
    fn get_z_mut(&mut self) -> &mut V {
        &mut self.pos.2
    }

    fn get_z(&self) -> V {
        self.pos.2
    }

    fn set_z(&mut self, value: V) {
        self.pos.2 = value;
    }
}

/// A four-dimensional position with an associated shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos4D<V, S: Shape<V>> {
    /// The position along the x, y, z, and w coordinate
    pub pos: (V, V, V, V),

    /// The given shape
    pub shape: S,
}

impl<V: Copy, S: Shape<V>> Pos4D<V, S> {
    #[must_use]
    /// Get the current pos
    pub const fn get_pos(&self) -> (V, V, V, V) {
        self.pos
    }
}
impl<V, S: Shape<V>> Pos4D<V, S> {
    #[must_use]
    /// Creates a new 4D position from coordinates and shape.
    pub const fn new(pos: (V, V, V, V), shape: S) -> Self {
        Self {
            pos,
            shape,
        }
    }
}

impl<V: Copy, S: Shape<V>> const GetPosX<V> for Pos4D<V, S> {
    fn get_x_mut(&mut self) -> &mut V {
        &mut self.pos.0
    }

    fn get_x(&self) -> V {
        self.pos.0
    }

    fn set_x(&mut self, value: V) {
        self.pos.0 = value;
    }
}

impl<V: Copy, S: Shape<V>> const GetPosY<V> for Pos4D<V, S> {
    fn get_y_mut(&mut self) -> &mut V {
        &mut self.pos.1
    }

    fn get_y(&self) -> V {
        self.pos.1
    }

    fn set_y(&mut self, value: V) {
        self.pos.1 = value;
    }
}

impl<V: Copy, S: Shape<V>> const GetPosZ<V> for Pos4D<V, S> {
    fn get_z_mut(&mut self) -> &mut V {
        &mut self.pos.2
    }

    fn get_z(&self) -> V {
        self.pos.2
    }

    fn set_z(&mut self, value: V) {
        self.pos.2 = value;
    }
}

impl<V: Copy, S: Shape<V>> const GetPosW<V> for Pos4D<V, S> {
    fn get_w_mut(&mut self) -> &mut V {
        &mut self.pos.3
    }

    fn get_w(&self) -> V {
        self.pos.3
    }

    fn set_w(&mut self, value: V) {
        self.pos.3 = value;
    }
}

impl<V, S: Shape<V>> const Position<V, S> for Pos1D<V, S> {
    fn get_shape(&self) -> &S {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut S {
        &mut self.shape
    }
}

impl<V, S: Shape<V>> const Position<V, S> for Pos2D<V, S> {
    fn get_shape(&self) -> &S {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut S {
        &mut self.shape
    }
}

impl<V, S: Shape<V>> const Position<V, S> for Pos3D<V, S> {
    fn get_shape(&self) -> &S {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut S {
        &mut self.shape
    }
}

impl<V, S: Shape<V>> const Position<V, S> for Pos4D<V, S> {
    fn get_shape(&self) -> &S {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut S {
        &mut self.shape
    }
}
