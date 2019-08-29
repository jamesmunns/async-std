use crate::stream::{FromStream, FromStreamFuture, IntoStream};

use cfg_if::cfg_if;

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

impl<T: Unpin + Sized> FromStream<T> for Vec<T> {
    #[inline]
    fn from_stream<S: IntoStream<Item = T>>(_stream: S) -> ret!(FromStreamFuture, Vec<T>) {
        unimplemented!();
    }
}
