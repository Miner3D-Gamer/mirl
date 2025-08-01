max = 32


code = "#![allow(missing_docs)]#![allow(clippy::unwrap_used)]#![allow(clippy::type_complexity)]use num_traits::{Num, NumCast};"


def get_code_for_trait_length(length: int):
    return "pub trait Tuple%sInto{fn tuple_%s_into<%s>(self)->(%s);}" % (
        length,
        length,
        ",".join(["T%s:Num+NumCast" % idx for idx in range(length)]),
        ",".join(["T%s" % idx for idx in range(length)]),
    )


# Allows the conversion of tuples with a length of %s
# Convert a tuple of X to a tuple of Y


def get_impl_for_trait_length(length: int):
    impl = "impl<%s>" % (",".join(["N" + str(i) for i in range(1, length + 1)]))
    for_ = "Tuple%sInto for(%s)" % (
        length,
        ",".join(["N" + str(i) for i in range(1, length + 1)]),
    )
    where = "where %s" % (
        ",".join(["N" + str(i) + ":Num+NumCast" for i in range(1, length + 1)])
    )
    header = "fn tuple_%s_into<%s>(self)->(%s)" % (
        length,
        ",".join(["T%s:Num+NumCast" % idx for idx in range(length)]),
        ",".join(["T%s" % idx for idx in range(length)]),
    )
    body = "({})".format(
        ",".join(
            [
                "NumCast::from(self.{}).unwrap()".format(i - 1)
                for i in range(1, length + 1)
            ]
        )
    )
    code = impl + for_ + where + " {" + header + " {" + body + "}" + "}"
    return code


original = """impl<N1, N2> Tuple2Into for (N1, N2)
where
    N1: Num + NumCast,
    N2: Num + NumCast,
{
    fn tuple_2_into<T: Num + NumCast>(self) -> (T, T) {
        (NumCast::from(self.0).unwrap(), NumCast::from(self.1).unwrap())
    }
}"""
final = ""
final += code
final += "pub trait Tuple1Into{fn tuple_1_into<T:Num+NumCast>(self)->(T,);}impl<N1>Tuple1Into for(N1,)where N1:Num+NumCast {fn tuple_1_into<T:Num+NumCast>(self)->(T,) {(NumCast::from(self.0).unwrap(),)}}"
for i in range(1, max + 1):
    i += 1
    final += get_code_for_trait_length(i)
    final += get_impl_for_trait_length(i)

import os

this = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(this, "conversion.rs"), "w") as f:
    f.write(final)
