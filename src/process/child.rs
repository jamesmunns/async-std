use std::ffi::OsStr;
use std::mem;
use std::path::Path;
use std::pin::Pin;

use crate::future::Future;
use crate::io;
use crate::process::{ExitStatus, Output};
use crate::task::blocking;
use crate::task::{Context, Poll};

pub struct Child {
    id: u32,
    state: State,
}

enum State {
    Idle(std::process::Child),
    Waiting(blocking::JoinHandle<io::Result<ExitStatus>>),
    Exited,
}

impl Child {
    pin_utils::unsafe_unpinned!(state: State);

    pub(crate) fn new(inner: std::process::Child) -> Child {
        Child {
            id: inner.id(),
            state: State::Idle(inner),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub async fn output(self) -> io::Result<Output> {
        match self.state {
            State::Idle(inner) => {
                blocking::spawn(async move { inner.wait_with_output() }).await
            }
            State::Waiting(handle) => {
                // TODO(stjepang): Handle this case gracefully. We should be able to await the
                // output even if the `Child` future was polled but not polled to completion.
                panic!("awaiting the output of a `Child` that has been polled");
            }
            State::Exited => panic!("awaiting the output of a completed process"),
        }
    }
}

impl Future for Child {
    type Output = io::Result<ExitStatus>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let state = self.state();
        loop {
            match state {
                State::Idle(_) => {
                    if let State::Idle(mut inner) = mem::replace(state, State::Exited) {
                        *state = State::Waiting(blocking::spawn(async move { inner.wait() }));
                    } else {
                        unreachable!()
                    }
                }
                State::Waiting(handle) => match Pin::new(handle).poll(cx) {
                    Poll::Ready(res) => {
                        *state = State::Exited;
                        return Poll::Ready(res);
                    }
                    Poll::Pending => return Poll::Pending,
                },
                State::Exited => panic!("awaiting the exit status of a completed process"),
            }
        }
    }
}
