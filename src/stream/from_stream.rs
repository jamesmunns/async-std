use cfg_if::cfg_if;

use super::IntoStream;

cfg_if! {
    if #[cfg(feature = "docs")] {
        #[doc(hidden)]
        pub struct ImplFuture<'a, T>(std::marker::PhantomData<T>);

        macro_rules! ret {
            ($f:tt, $o:ty) => (ImplFuture<$o>);
        }
    } else {
        macro_rules! ret {
            ($f:tt, $o:ty) => ($f<Self>);
        }
    }
}

/// Conversion from a `Stream`.
///
/// By implementing `FromStream` for a type, you define how it will be created from a stream.
/// This is common for types which describe a collection of some kind.
///
/// See also: [`IntoStream`].
///
/// [`IntoStream`]: trait.IntoStream.html
pub trait FromStream<T>: Sized + Unpin {
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
    fn from_stream<S: IntoStream<Item = T>>(stream: S) -> ret!(FromStreamFuture, Self);
}

#[doc(hidden)]
#[allow(missing_debug_implementations)]
#[allow(unused)]
pub struct FromStreamFuture<S: Unpin + ?Sized> {
    stream: S
}
