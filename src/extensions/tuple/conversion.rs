
pub trait ConvertTuple<T> {
    type Output;
    fn convert_to(self) -> Self::Output;
}

impl<A, T> ConvertTuple<T> for (A,)
where
    A: Into<T>,
{
    type Output = (T,);
    fn convert_to(self) -> Self::Output {
        (self.0.into(),)
    }
}

impl<A, B, T> ConvertTuple<T> for (A, B)
where
    A: Into<T>,
    B: Into<T>,
{
    type Output = (T, T);
    fn convert_to(self) -> Self::Output {
        (self.0.into(), self.1.into())
    }
}

impl<A, B, C, T> ConvertTuple<T> for (A, B, C)
where
    A: Into<T>,
    B: Into<T>,
    C: Into<T>,
{
    type Output = (T, T, T);
    fn convert_to(self) -> Self::Output {
        (self.0.into(), self.1.into(), self.2.into())
    }
}

impl<A, B, C, D, T> ConvertTuple<T> for (A, B, C, D)
where
    A: Into<T>,
    B: Into<T>,
    C: Into<T>,
    D: Into<T>,
{
    type Output = (T, T, T, T);
    fn convert_to(self) -> Self::Output {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into())
    }
}

impl<A, B, C, D, E, T> ConvertTuple<T> for (A, B, C, D, E)
where
    A: Into<T>,
    B: Into<T>,
    C: Into<T>,
    D: Into<T>,
    E: Into<T>,
{
    type Output = (T, T, T, T, T);
    fn convert_to(self) -> Self::Output {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into())
    }
}

impl<A, B, C, D, E, F, T> ConvertTuple<T> for (A, B, C, D, E, F)
where
    A: Into<T>,
    B: Into<T>,
    C: Into<T>,
    D: Into<T>,
    E: Into<T>,
    F: Into<T>,
{
    type Output = (T, T, T, T, T, T);
    fn convert_to(self) -> Self::Output {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into())
    }
}