//! Cross platform code

use std::path::Path;
use std::process::Command;

pub fn open_folder(p: impl AsRef<Path>) -> anyhow::Result<()> {
    const CMD: &str = if cfg!(target_os = "windows") {
        "explorer"
    } else if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "linux") {
        "xdg-open"
    } else {
        unimplemented!()
    };

    Command::new(CMD).arg(p.as_ref()).spawn()?;
    Ok(())
}
