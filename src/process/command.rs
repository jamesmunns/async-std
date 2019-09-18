use std::ffi::OsStr;
use std::mem;
use std::path::Path;
use std::pin::Pin;

use crate::future::Future;
use crate::io;
use crate::process::{Child, ExitStatus, Output};
use crate::task::blocking;
use crate::task::{Context, Poll};

/// A builder for spawning processes.
///
/// This type is an async version of [`std::process::Command`].
///
/// [`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
///
/// A default configuration can be
/// generated using `Command::new(program)`, where `program` gives a path to the
/// program to be executed. Additional builder methods allow the configuration
/// to be changed (for example, by adding arguments) prior to spawning:
///
/// ```
/// use std::process::Command;
///
/// let output = if cfg!(target_os = "windows") {
///     Command::new("cmd")
///             .args(&["/C", "echo hello"])
///             .output()
///             .expect("failed to execute process")
/// } else {
///     Command::new("sh")
///             .arg("-c")
///             .arg("echo hello")
///             .output()
///             .expect("failed to execute process")
/// };
///
/// let hello = output.stdout;
/// ```
///
/// `Command` can be reused to spawn multiple processes. The builder methods
/// change the command without needing to immediately spawn the process.
///
/// ```no_run
/// use std::process::Command;
///
/// let mut echo_hello = Command::new("sh");
/// echo_hello.arg("-c")
///           .arg("echo hello");
/// let hello_1 = echo_hello.output().expect("failed to execute process");
/// let hello_2 = echo_hello.output().expect("failed to execute process");
/// ```
///
/// Similarly, you can call builder methods after spawning a process and then
/// spawn a new process with the modified settings.
///
/// ```no_run
/// use std::process::Command;
///
/// let mut list_dir = Command::new("ls");
///
/// // Execute `ls` in the current directory of the program.
/// list_dir.status().expect("process failed to execute");
///
/// println!("");
///
/// // Change `ls` to execute in the root directory.
/// list_dir.current_dir("/");
///
/// // And then execute `ls` again but in the root directory.
/// list_dir.status().expect("process failed to execute");
/// ```
pub struct Command(std::process::Command);

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command(std::process::Command::new(program))
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.0.arg(arg);
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.env(key, val);
        self
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Command
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.envs(vars);
        self
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Command {
        self.0.env_remove(key);
        self
    }

    pub fn env_clear(&mut self) -> &mut Command {
        self.0.env_clear();
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command {
        self.0.current_dir(dir);
        self
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        let inner = self.0.spawn()?;
        Ok(Child::new(inner))
    }

    // pub async fn output(&mut self) -> io::Result<Output> {
    //     self.spawn()?.output().await
    // }

    pub async fn status(&mut self) -> io::Result<ExitStatus> {
        self.spawn()?.await
    }
}
