

pub fn concatenate<T: AsRef<str>>(input1: T, input2: T) -> String {
    let str1 = input1.as_ref();
    let str2 = input2.as_ref();
    format!("{}{}", str1, str2)
}

