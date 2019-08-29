use crate::stream::{FromStream, IntoStream, Stream};

use std::pin::Pin;

impl<T: Unpin + Sized + Send> FromStream<T> for Vec<T> {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = T>> (
        stream: S,
    ) -> Pin<Box<dyn core::future::Future<Output = Self> + Send + 'a>>
        where <S as IntoStream>::IntoStream: Send + 'a,
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
