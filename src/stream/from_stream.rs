use super::IntoStream;

/// Conversion from a `Stream`.
///
/// By implementing `FromStream` for a type, you define how it will be created from a stream.
/// This is common for types which describe a collection of some kind.
///
/// See also: [`FromStream`].
///
/// [`FromStream`]: trait.FromStream.html
pub trait FromStream<A>: Sized {
    /// Creates a value from a stream.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // use async_std::stream::FromStream;
    ///
    /// // let _five_fives = async_std::stream::repeat(5).take(5);
    /// ```
    fn from_stream<T: IntoStream<Item = A>>(iter: T) -> Self;
}
