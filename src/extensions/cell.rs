use std::cell::Cell;

/// Set the value inside the Cell to zero
pub trait SetCellToZero {
    /// Zero out cell's value
    fn set_zero(&self);
}
impl<T: num_traits::ConstZero> SetCellToZero for std::cell::Cell<T> {
    fn set_zero(&self) {
        self.set(T::ZERO);
    }
}

/// Basic arithmetic operations that modify the cell in place.
pub trait CellIntOps<T> {
    /// Adds `rhs` to the cell's value.
    fn add(&self, rhs: T);
    /// Subtracts `rhs` from the cell's value.
    fn sub(&self, rhs: T);
    /// Multiplies the cell's value by `rhs`.
    fn mul(&self, rhs: T);
    /// Divides the cell's value by `rhs`.
    fn div(&self, rhs: T);
}

/// Basic arithmetic operations that return a new value without modifying the cell.
pub trait CellIntReturnOps<T> {
    /// Returns the cell's value plus `rhs`.
    fn added(&self, rhs: T) -> T;
    /// Returns the cell's value minus `rhs`.
    fn subtracted(&self, rhs: T) -> T;
    /// Returns the cell's value multiplied by `rhs`.
    fn multiplied(&self, rhs: T) -> T;
    /// Returns the cell's value divided by `rhs`.
    fn divided(&self, rhs: T) -> T;
}

/// Saturating arithmetic operations that modify the cell in place.
pub trait CellIntSaturatedOps<T> {
    /// Adds `rhs` to the cell's value, saturating at numeric bounds.
    fn saturating_add(&self, rhs: T);
    /// Subtracts `rhs` from the cell's value, saturating at numeric bounds.
    fn saturating_sub(&self, rhs: T);
    /// Multiplies the cell's value by `rhs`, saturating at numeric bounds.
    fn saturating_mul(&self, rhs: T);
}

/// Saturating arithmetic operations that return a new value without modifying the cell.
pub trait CellIntSaturatedReturnOps<T> {
    /// Returns the cell's value plus `rhs`, saturating at numeric bounds.
    fn saturating_added(&self, rhs: T) -> T;
    /// Returns the cell's value minus `rhs`, saturating at numeric bounds.
    fn saturating_subtracted(&self, rhs: T) -> T;
    /// Returns the cell's value multiplied by `rhs`, saturating at numeric bounds.
    fn saturating_multiplied(&self, rhs: T) -> T;
}

impl<T> CellIntOps<T> for Cell<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    fn add(&self, rhs: T) {
        self.set(self.get() + rhs);
    }

    fn sub(&self, rhs: T) {
        self.set(self.get() - rhs);
    }

    fn mul(&self, rhs: T) {
        self.set(self.get() * rhs);
    }

    fn div(&self, rhs: T) {
        self.set(self.get() / rhs);
    }
}

impl<T> CellIntReturnOps<T> for Cell<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    fn added(&self, rhs: T) -> T {
        let result = self.get() + rhs;
        self.set(result);
        result
    }

    fn subtracted(&self, rhs: T) -> T {
        let result = self.get() - rhs;
        self.set(result);
        result
    }

    fn multiplied(&self, rhs: T) -> T {
        let result = self.get() * rhs;
        self.set(result);
        result
    }

    fn divided(&self, rhs: T) -> T {
        let result = self.get() / rhs;
        self.set(result);
        result
    }
}

macro_rules! impl_saturated {
    ($($ty:ty),*) => {
        $(
            impl CellIntSaturatedOps<$ty> for Cell<$ty> {
                fn saturating_add(&self, rhs: $ty) {
                    self.set(self.get().saturating_add(rhs));
                }

                fn saturating_sub(&self, rhs: $ty) {
                    self.set(self.get().saturating_sub(rhs));
                }

                fn saturating_mul(&self, rhs: $ty) {
                    self.set(self.get().saturating_mul(rhs));
                }
            }

            impl CellIntSaturatedReturnOps<$ty> for Cell<$ty> {
                fn saturating_added(&self, rhs: $ty) -> $ty {
                    let result = self.get().saturating_add(rhs);
                    //self.set(result);
                    result
                }

                fn saturating_subtracted(&self, rhs: $ty) -> $ty {
                    let result = self.get().saturating_sub(rhs);
                    //self.set(result);
                    result
                }

                fn saturating_multiplied(&self, rhs: $ty) -> $ty {
                    let result = self.get().saturating_mul(rhs);
                    //self.set(result);
                    result
                }
            }
        )*
    };
}

/// Clamping operations that modify the cell in place.
pub trait CellIntClampingOps<T> {
    /// Sets the cell's value to the minimum of its current value and `rhs`.
    fn min(&self, rhs: T);
    /// Sets the cell's value to the maximum of its current value and `rhs`.
    fn max(&self, rhs: T);
    /// Clamps the cell's value between `min` and `max`.
    fn clamp(&self, min: T, max: T);
}

/// Clamping operations that return a new value without modifying the cell.
pub trait CellIntClampingReturnOps<T> {
    /// Returns the minimum of the cell's value and `rhs`.
    fn minned(&self, rhs: T) -> T;
    /// Returns the maximum of the cell's value and `rhs`.
    fn maxed(&self, rhs: T) -> T;
    /// Returns the cell's value clamped between `min` and `max`.
    fn clamped(&self, min: T, max: T) -> T;
}

/// Sign-related operations that modify the cell in place.
pub trait CellIntSignedOps<T> {
    /// Sets the cell's value to its absolute value.
    fn abs(&self);
    /// Negates the cell's value.
    fn neg(&self);
    /// Sets the cell's value to its sign (-1, 0, or 1).
    fn signum(&self);
}

/// Sign-related operations that return a new value without modifying the cell.
pub trait CellIntSignedReturnOps<T> {
    /// Returns the absolute value of the cell's value.
    fn absed(&self) -> T;
    /// Returns the negation of the cell's value.
    fn neged(&self) -> T;
    /// Returns the sign of the cell's value (-1, 0, or 1).
    fn signumed(&self) -> T;
}

/// Saturating sign-related operations that modify the cell in place.
pub trait CellIntSaturatedSignedOps<T> {
    /// Sets the cell's value to its absolute value, saturating at numeric bounds.
    fn saturating_abs(&self);
    /// Negates the cell's value, saturating at numeric bounds.
    fn saturating_neg(&self);
}

/// Saturating sign-related operations that return a new value without modifying the cell.
pub trait CellIntSaturatedSignedReturnOps<T> {
    /// Returns the absolute value of the cell's value, saturating at numeric bounds.
    fn saturating_absed(&self) -> T;
    /// Returns the negation of the cell's value, saturating at numeric bounds.
    fn saturating_neged(&self) -> T;
}

impl<T> CellIntClampingOps<T> for Cell<T>
where
    T: Copy + Ord,
{
    fn min(&self, rhs: T) {
        self.set(self.get().min(rhs));
    }

    fn max(&self, rhs: T) {
        self.set(self.get().max(rhs));
    }

    fn clamp(&self, min: T, max: T) {
        self.set(self.get().clamp(min, max));
    }
}

impl<T> CellIntClampingReturnOps<T> for Cell<T>
where
    T: Copy + Ord,
{
    fn minned(&self, rhs: T) -> T {
        self.get().min(rhs)
    }

    fn maxed(&self, rhs: T) -> T {
        
        //self.set(result);
        self.get().max(rhs)
    }

    fn clamped(&self, min: T, max: T) -> T {
        
        //self.set(result);
        self.get().clamp(min, max)
    }
}

macro_rules! impl_signed {
    ($($ty:ty),*) => {
        $(
            impl CellIntSignedOps<$ty> for Cell<$ty> {
                fn abs(&self) {
                    self.set(self.get().abs());
                }

                fn neg(&self) {
                    self.set(-self.get());
                }

                fn signum(&self) {
                    self.set(self.get().signum());
                }
            }

            impl CellIntSignedReturnOps<$ty> for Cell<$ty> {
                fn absed(&self) -> $ty {
                    let result = self.get().abs();
                    //self.set(result);
                    result
                }

                fn neged(&self) -> $ty {
                    let result = -self.get();
                    //self.set(result);
                    result
                }

                fn signumed(&self) -> $ty {
                    let result = self.get().signum();
                    //self.set(result);
                    result
                }
            }

            impl CellIntSaturatedSignedOps<$ty> for Cell<$ty> {
                fn saturating_abs(&self) {
                    self.set(self.get().saturating_abs());
                }

                fn saturating_neg(&self) {
                    self.set(self.get().saturating_neg());
                }
            }

            impl CellIntSaturatedSignedReturnOps<$ty> for Cell<$ty> {
                fn saturating_absed(&self) -> $ty {
                    let result = self.get().saturating_abs();
                    //self.set(result);
                    result
                }

                fn saturating_neged(&self) -> $ty {
                    let result = self.get().saturating_neg();
                    //self.set(result);
                    result
                }
            }
        )*
    };
}

impl_saturated!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_signed!(i8, i16, i32, i64, i128, isize);
