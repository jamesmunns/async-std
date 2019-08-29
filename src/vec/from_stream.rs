use futures::ready;
use futures::stream::{Stream, StreamExt};

use crate::stream::{FromStream, IntoStream};

use std::pin::Pin;

impl<T: Unpin + Sized> FromStream<T> for Vec<T> {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = T>>(
        stream: S,
    ) -> Pin<Box<dyn core::future::Future<Output = Self> + Send + 'a>> {
        Pin::from(Box::new(async move {
            let mut out = vec![];
            let mut stream = stream.into_stream();
            while let Some(item) = stream.next().await {
                out.push(item);
            }
            out
        }))
    }
}
