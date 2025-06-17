import os

max_size = 32

# Macro
code_macro = """use std::ops::{Add,Div,Mul,Sub};pub trait TupleOps<Rhs=Self>{type Output;fn add(self,rhs:Rhs)->Self::Output;fn sub(self,rhs:Rhs)->Self::Output;fn mul(self,rhs:Rhs)->Self::Output;fn div(self,rhs:Rhs)->Self::Output;}macro_rules!tuple_ops{($($n:tt),+)=>{impl<T>TupleOps for($(impl_helper!(@type$n T),)+)where T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Div<Output=T>+Copy{type Output=($(impl_helper!(@type$n T),)+);fn add(self,rhs:Self)->Self::Output{($(self.$n+rhs.$n,)+)}fn sub(self,rhs:Self)->Self::Output{($(self.$n-rhs.$n,)+)}fn mul(self,rhs:Self)->Self::Output{($(self.$n*rhs.$n,)+)}fn div(self,rhs:Self)->Self::Output{($(self.$n/rhs.$n,)+)}}};}macro_rules!impl_helper{(@type $_:tt$t:ty)=>($t);}"""

for size in range(2, max_size + 1):
    indices = ','.join([str(i) for i in range(size)])
    code_macro += f"tuple_ops!{{{indices}}}"

script_dir = os.path.dirname(os.path.abspath(__file__))
output_file = os.path.join(script_dir, "tuple.rs")

with open(output_file, "w") as f:
    f.write(code_macro)