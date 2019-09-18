use async_std::task;
use async_std::process;
use async_std::io;

fn main() -> io::Result<()> {
    task::block_on(async {
        dbg!(process::Command::new("ls").spawn()?.await?);
        Ok(())
    })
}
