// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
// /// A shape with a x dimensional position
// pub struct Position<POS, V, S: Shape<V>> {
//     pos: POS,
//     shape: S,
//     raw_value_type: core::marker::PhantomData<V>,
// }
// impl<T, V, S: Shape<V>> Position<T, V, S> {
//     /// Create a new positioned shape
//     pub const fn new(pos: T, shape: S) -> Self {
//         Self {
//             pos,
//             shape,
//             raw_value_type: core::marker::PhantomData,
//         }
//     }
// }
// impl<T: Copy, S: Shape<T>> Position<(T,), T, S> {
//     /// Returns the x position
//     pub const fn get_x(&self) -> T {
//         self.pos.0
//     }

//     /// Sets the x position
//     pub const fn set_x(&mut self, value: T) {
//         self.pos.0 = value;
//     }
// }

// impl<T: Copy, S: Shape<T>> Position<(T, T), T, S> {
//     /// Returns the x position
//     pub const fn get_x(&self) -> T {
//         self.pos.0
//     }
//     /// Returns the y position
//     pub const fn get_y(&self) -> T {
//         self.pos.1
//     }

//     /// Sets the x position
//     pub const fn set_x(&mut self, value: T) {
//         self.pos.0 = value;
//     }
//     /// Sets the y position
//     pub const fn set_y(&mut self, value: T) {
//         self.pos.1 = value;
//     }
// }

// impl<T: Copy, S: Shape<T>> Position<(T, T, T), T, S> {
//     /// Returns the x position
//     pub const fn get_x(&self) -> T {
//         self.pos.0
//     }
//     /// Returns the y position
//     pub const fn get_y(&self) -> T {
//         self.pos.1
//     }
//     /// Returns the z position
//     pub const fn get_z(&self) -> T {
//         self.pos.2
//     }

//     /// Sets the x position
//     pub const fn set_x(&mut self, value: T) {
//         self.pos.0 = value;
//     }
//     /// Sets the y position
//     pub const fn set_y(&mut self, value: T) {
//         self.pos.1 = value;
//     }
//     /// Sets the z position
//     pub const fn set_z(&mut self, value: T) {
//         self.pos.2 = value;
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
// /// A shape with a x dimensional position and an x dimensional rotation
// pub struct Rotation<R, T, V, S: Shape<V>> {
//     position: Position<T, V, S>,
//     rotation: R,
// }
