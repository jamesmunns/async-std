use super::Stream;

/// Conversion into a `Stream`.
///
/// By implementing `FromStream` for a type, you define how it will be
/// created from a stream. This is common for types which describe a
/// collection of some kind.
///
/// [`from_stream`]: #tymethod.from_iter
/// [`Stream`]: trait.Stream.html
/// [`collect`]: trait.Iterator.html#method.collect
///
/// See also: [`IntoStream`].
///
/// [`IntoStream`]: trait.IntoStream.html
pub trait IntoStream {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of stream are we turning this into?
    type IntoStream: Stream;

    /// Creates a stream from a value.
    fn into_stream(self) -> Self::IntoStream;
}
