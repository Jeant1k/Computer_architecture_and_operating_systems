use std::{error::Error, ffi::CString, io::{self, prelude::*}, os::fd::AsRawFd, str, vec};

use nix::{
    errno::Errno, fcntl::{open, OFlag}, libc::{self, STDIN_FILENO, STDOUT_FILENO}, sys::{stat::Mode, wait::waitpid}, unistd::{close, dup2, execvp, fork, pipe, write, ForkResult}
};

fn main() -> Result<(), Box<dyn Error>> {
    let result = unsafe {
        fork()?
    };
    match result {
        ForkResult::Parent { child } => {
            println!("child PID = {}", child);
            waitpid(child, None)?;
        }
        ForkResult::Child => {
            let fd = open(
                "output.txt",
                OFlag::O_WRONLY | OFlag::O_CREAT,
                Mode::S_IRWXU
            )?;

            unsafe {
                Errno::result(libc::dup2(fd.as_raw_fd(), STDOUT_FILENO))?;
            }

            let cmd = CString::new("echo")?;
            let args = vec![cmd.clone(), CString::new("hi\n")?];
            execvp(&cmd, &args)?;
        }
    }

    Ok(())
}
