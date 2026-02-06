use crate::math::collision::{Circle, Rectangle};
#[derive(Debug, Clone, PartialEq, Eq)]
/// A collection of supported objects
pub struct Scene2D<T, const CS: bool> {
    objects: Vec<SupportedShapes2D<T, CS>>,
    positions: Vec<(T, T)>,
    rotations: Vec<(T, T)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
/// All supported shapes
pub enum SupportedShapes2D<T, const CS: bool> {
    Rectangle(Rectangle<T, CS>),
    Circle(Circle<T, CS>),
    Point((T, T)),
}
// impl<T, const CS: bool> SupportedShapes2D<T, CS> {
//     /// This assumes that this shape is at fault for the collision
//     pub fn collide(&mut self, other: &Self) {
//         match self {
//             Self::Circle(s1) => match other {
//                 Self::Circle(s2) => circle_circle(s1, s2),
//                 Self::Rectangle(s2) => circle_rectangle(s1, s2),
//                 Self::Point(s2) => circle_point(s1, s2),
//             },
//             Self::Rectangle(s1) => match other {
//                 Self::Circle(s2) => rectangle_circle(s1, s2),
//                 Self::Rectangle(s2) => rectangle_rectangle(s1, s2),
//                 Self::Point(s2) => rectangle_point(s1, s2),
//             },
//             Self::Point(s1) => match other {
//                 Self::Circle(s2) => point_circle(s1, s2),
//                 Self::Rectangle(s2) => point_rectangle(s1, s2),
//                 Self::Point(s2) => point_point::<T, CS>(s1, s2),
//             },
//         }
//     }
// }
// #[allow(unused, clippy::nursery)]
// fn rectangle_circle<T, const CS: bool>(
//     rectangle: &mut Rectangle<T, CS>,
//     circle: &Circle<T, CS>,
// ) {
//     if rectangle.(other)
// }
// #[allow(unused, clippy::nursery)]
// fn rectangle_rectangle<T, const CS: bool>(
//     rectangle: &mut Rectangle<T, CS>,
//     rectangle2: &Rectangle<T, CS>,
// ) {
// }
// #[allow(unused, clippy::nursery)]
// fn circle_circle<T, const CS: bool>(
//     circle: &mut Circle<T, CS>,
//     circle2: &Circle<T, CS>,
// ) {
// }

// #[allow(unused, clippy::nursery)]
// fn circle_rectangle<T, const CS: bool>(
//     circle: &mut Circle<T, CS>,
//     rectangle: &Rectangle<T, CS>,
// ) {
// }
// #[allow(unused, clippy::nursery)]
// fn rectangle_point<T, const CS: bool>(
//     rectangle: &mut Rectangle<T, CS>,
//     point: &(T, T),
// ) {
// }
// #[allow(unused, clippy::nursery)]
// fn point_point<T, const CS: bool>(point: &mut (T, T), point2: &(T, T)) {}

// #[allow(unused, clippy::nursery)]
// fn point_rectangle<T, const CS: bool>(
//     point: &mut (T, T),
//     rectangle: &Rectangle<T, CS>,
// ) {
// }

// #[allow(unused, clippy::nursery)]
// fn circle_point<T, const CS: bool>(circle: &mut Circle<T, CS>, point: &(T, T)) {
// }

// #[allow(unused, clippy::nursery)]
// fn point_circle<T, const CS: bool>(point: &(T, T), circle: &Circle<T, CS>) {}
