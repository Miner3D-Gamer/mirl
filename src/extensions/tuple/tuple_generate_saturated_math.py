import os

max_size = 32

# Macro with saturating operations
code_macro = """#![allow(missing_docs)]
use num_traits::{SaturatingAdd, SaturatingSub, SaturatingMul};
use std::ops::Div;

pub trait TupleOps<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
    fn mul(self, rhs: Rhs) -> Self::Output;
    fn div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! tuple_ops {
    ($($n:tt),+) => {
        impl<T> TupleOps for ($(impl_helper!(@type $n T),)+)
        where
            T: SaturatingAdd<Output = T> + SaturatingSub<Output = T> + SaturatingMul<Output = T> + Div<Output = T> + Copy
        {
            type Output = ($(impl_helper!(@type $n T),)+);
            
            fn add(self, rhs: Self) -> Self::Output {
                ($(self.$n.saturating_add(&rhs.$n),)+)
            }
            
            fn sub(self, rhs: Self) -> Self::Output {
                ($(self.$n.saturating_sub(&rhs.$n),)+)
            }
            
            fn mul(self, rhs: Self) -> Self::Output {
                ($(self.$n.saturating_mul(&rhs.$n),)+)
            }
            
            fn div(self, rhs: Self) -> Self::Output {
                ($(self.$n / rhs.$n,)+)
            }
        }
    };
}

macro_rules! impl_helper {
    (@type $_:tt $t:ty) => ($t);
}

"""

for size in range(2, max_size + 1):
    indices = ','.join([str(i) for i in range(size)])
    code_macro += f"tuple_ops!{{{indices}}};\n"

script_dir = os.path.dirname(os.path.abspath(__file__))
output_file = os.path.join(script_dir, "tuple.rs")

with open(output_file, "w") as f:
    f.write(code_macro)

print(f"Generated saturating tuple operations in {output_file}")