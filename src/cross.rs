//! Cross platform code

use std::path::Path;
use std::process::Command;

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
