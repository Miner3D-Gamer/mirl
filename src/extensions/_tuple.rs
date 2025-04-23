use std::ops::{Add, Div, Mul, Sub};

pub trait TupleOps<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
    fn mul(self, rhs: Rhs) -> Self::Output;
    fn div(self, rhs: Rhs) -> Self::Output;
}

impl<T> TupleOps for (T, T)
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy,
{
    type Output = (T, T);

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1)
    }

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0, self.1 - rhs.1)
    }

    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.0, self.1 * rhs.1)
    }

    fn div(self, rhs: Self) -> Self::Output {
        (self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<T> TupleOps for (T, T, T)
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy,
{
    type Output = (T, T, T);

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }

    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }

    fn div(self, rhs: Self) -> Self::Output {
        (self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
