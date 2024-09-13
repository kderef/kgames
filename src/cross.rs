//! Cross platform code

use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub fn open_path(p: impl AsRef<Path>) -> anyhow::Result<()> {
    let (cmd, args): (&str, &[&str]) = if cfg!(target_os = "windows") {
        ("cmd.exe", &["/C", "start"])
    } else if cfg!(target_os = "macos") {
        ("open", &[])
    } else if cfg!(target_os = "linux") {
        ("xdg-open", &[])
    } else {
        unimplemented!()
    };

    Command::new(cmd).args(args).arg(p.as_ref()).spawn()?;
    Ok(())
}

/// Shortcut to run a command
#[inline]
#[allow(unused)]
pub fn cmd<S: AsRef<OsStr>>(
    name: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = S>,
) -> anyhow::Result<ExitStatus> {
    Command::new(name.as_ref())
        .args(args)
        .status()
        .map_err(Into::into)
}
