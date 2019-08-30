use cfg_if::cfg_if;

use crate::stream::{FromStream, IntoStream, Stream};

use std::pin::Pin;

cfg_if! {
    if #[cfg(feature = "docs")] {
        #[doc(hidden)]
        pub struct DynFuture<'a, T>(std::marker::PhantomData<&'a T>);

        macro_rules! dyn_ret {
            ($a:lifetime, $o:ty) => (DynFuture<$a, $o>);
        }
    } else {
        macro_rules! dyn_ret {
            ($a:lifetime, $o:ty) => (Pin<Box<dyn core::future::Future<Output = $o> + Send + 'a>>)
        }
    }
}

impl<T: Unpin + Sized + Send> FromStream<T> for Vec<T> {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = T>>(
        stream: S,
    ) -> dyn_ret!('a, Self)
    where
        <S as IntoStream>::IntoStream: Send + 'a,
    {
        let mut stream = stream.into_stream();
        Pin::from(Box::new(async move {
            let mut out = vec![];
            while let Some(item) = stream.next().await {
                out.push(item);
            }
            out
        }))
    }
}
