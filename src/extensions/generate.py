import os

max_size = 16

code = """use std::ops::{Add,Div,Mul,Sub};pub trait TupleOps<Rhs=Self>{type Output;fn add(self,rhs:Rhs)->Self::Output;fn sub(self,rhs:Rhs)->Self::Output;fn mul(self,rhs:Rhs)->Self::Output;fn div(self,rhs:Rhs)->Self::Output;}"""

for size in range(2, max_size + 1):
    code += f"""impl<T>TupleOps for({','.join(['T'] * size)})where T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Div<Output=T>+Copy,{{type Output=({','.join(['T'] * size)});fn add(self,rhs:Self)->Self::Output{{({','.join([f'self.{i}+rhs.{i}' for i in range(size)])})}}fn sub(self,rhs:Self)->Self::Output{{({','.join([f'self.{i}-rhs.{i}' for i in range(size)])})}}fn mul(self,rhs:Self)->Self::Output{{({','.join([f'self.{i}*rhs.{i}' for i in range(size)])})}}fn div(self,rhs:Self)->Self::Output{{({','.join([f'self.{i}/rhs.{i}' for i in range(size)])})}}}}"""


script_dir = os.path.dirname(os.path.abspath(__file__))


output_file = os.path.join(script_dir, "tuple.rs")

with open(output_file, "w") as f:
    f.write(code)
