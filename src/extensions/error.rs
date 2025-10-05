#[allow(clippy::missing_errors_doc)]
/// Converts from one error type to another
pub fn result_error_into<T, I, O: std::convert::From<I>>(
    result: std::result::Result<T, I>,
) -> std::result::Result<T, O> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}

#[allow(clippy::missing_errors_doc)]
/// Converts from one value type to another
pub fn result_value_into<T, V: std::convert::From<T>, I>(
    result: std::result::Result<T, I>,
) -> std::result::Result<V, I> {
    match result {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(e),
    }
}

#[allow(clippy::missing_errors_doc)]
/// Converts from one value and error type to another
pub fn result_into<T, V: std::convert::From<T>, I, O: std::convert::From<I>>(
    result: std::result::Result<T, I>,
) -> std::result::Result<V, O> {
    match result {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(e.into()),
    }
}
#[const_trait]
/// Converts from one error type to another
pub trait ExtendedErrorErrorInto<T, I, O: std::convert::From<I>> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into(&self) -> std::result::Result<T, O>;
}

impl<T: Clone, I: Clone, O: std::convert::From<I>>
    ExtendedErrorErrorInto<T, I, O> for std::result::Result<T, I>
{
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into(&self) -> std::result::Result<T, O> {
        match self {
            Ok(v) => Ok(v.clone()),
            Err(e) => Err(e.clone().into()),
        }
    }
}

#[const_trait]
/// Converts from one value type inside an error to another
pub trait ExtendedErrorValueInto<T, V: std::convert::From<T>, I> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value type inside an error to another
    fn result_value_into(&self) -> std::result::Result<V, I>;
}

impl<T: Clone, V: std::convert::From<T>, I: Clone>
    ExtendedErrorValueInto<T, V, I> for std::result::Result<T, I>
{
    fn result_value_into(&self) -> std::result::Result<V, I> {
        match self {
            Ok(v) => Ok(v.clone().into()),
            Err(e) => Err(e.clone()),
        }
    }
}

#[const_trait]
/// Converts from one value and error type to another
pub trait ExtendedErrorFullInto<
    T,
    V: std::convert::From<T>,
    I,
    O: std::convert::From<I>,
>
{
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value and error type to another
    fn result_into(&self) -> std::result::Result<V, O>;
}

impl<
        T: Clone,
        V: std::convert::From<T>,
        I: Clone,
        O: std::convert::From<I>,
    > ExtendedErrorFullInto<T, V, I, O> for std::result::Result<T, I>
{
    fn result_into(&self) -> std::result::Result<V, O> {
        match self {
            Ok(v) => Ok(v.clone().into()),
            Err(e) => Err(e.clone().into()),
        }
    }
}
