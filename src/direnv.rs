use std::{path::Path, process::Command};
use thiserror::Error;

pub struct Direnv;

#[derive(Error, Debug)]
pub enum DirenvError {
    #[error("I/O Error: {0}")]
    Io(std::io::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8(std::str::Utf8Error),
    #[error("Command failed with status {status:?}: stderr")]
    CommandFailed {
        status: std::process::ExitStatus,
        stderr: String,
    },
    #[error("Invalid path (non-UTF-8)")]
    InvalidPath,
}

pub type Result<T> = std::result::Result<T, DirenvError>;

impl From<std::io::Error> for DirenvError {
    fn from(error: std::io::Error) -> Self {
        DirenvError::Io(error)
    }
}

impl From<std::str::Utf8Error> for DirenvError {
    fn from(error: std::str::Utf8Error) -> Self {
        DirenvError::Utf8(error)
    }
}

impl Direnv {
    pub fn allow(path: &Path) -> Result<()> {
        Command::new("direnv")
            .args(["allow", path.to_str().ok_or(DirenvError::InvalidPath)?])
            .output()?;
        Ok(())
    }
    pub fn block(path: &Path) -> Result<()> {
        Command::new("direnv")
            .args(["block", path.to_str()])
            .output()?;
        Ok(())
    }
    pub fn deny(path: &Path) -> Result<()> {
        Command::new("direnv")
            .args(["deny", path.to_str()])
            .output()?;
        Ok(())
    }
    pub fn edit(path: &Path) -> Result<()> {
        Command::new("direnv")
            .args(["edit", path.to_str()])
            .output()?;
        Ok(())
    }
    pub fn exec(path: &Path, cmd_args: &[&str]) -> Result<()> {
        Command::new("direnv")
            .args(["exec", path.to_str()])
            .args(cmd_args)
            .output()?;
        Ok(())
    }
    pub fn export(shell: &str) -> Result<()> {
        Command::new("direnv").args(["export", shell]).output()?;
        Ok(())
    }
    pub fn fetchurl(url: &str) -> Result<()> {
        Command::new("direnv").args(["fetchurl", url]).output()?;
        Ok(())
    }
    pub fn grant(path: &Path) -> Result<()> {
        Command::new("direnv")
            .args(["grant", path.to_str()])
            .output()?;
        Ok(())
    }
    pub fn hook(shell: &str) -> Result<()> {
        Command::new("direnv").args(["hook", shell]).output()?;
        Ok(())
    }
    pub fn permit(path: &Path) -> Result<()> {
        Command::new("direnv")
            .args(["permit", path.to_str()])
            .output()?;
        Ok(())
    }
    pub fn prune() -> Result<()> {
        Command::new("direnv").arg("prune").output()?;
        Ok(())
    }
    pub fn reload() -> Result<()> {
        Command::new("direnv").arg("prune").output()?;
        Ok(())
    }
    pub fn revoke(path: &Path) -> Result<()> {
        Command::new("direnv")
            .arg(["revoke", path.to_str()])
            .output()?;
        Ok(())
    }
    pub fn status() -> Result<()> {
        Command::new("direnv").arg("status").output()?;
        Ok(())
    }
    pub fn stdlib() -> Result<()> {
        Command::new("direnv").arg("status").output()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direnv_allow() -> Result<()> {
        Direnv::allow(Path::new(""))?;
        Ok(())
    }
}
