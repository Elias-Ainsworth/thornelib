use std::{path::Path, process::Command};
use thiserror::Error;

pub struct Direnv;

#[derive(Error, Debug)]
pub enum DirenvError {
    #[error("I/O Error: {0}")]
    Io(std::io::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8(std::str::Utf8Error),
    #[error("From UTF-8 Error: {0}")]
    FromUtf8(std::string::FromUtf8Error),
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

impl From<std::string::FromUtf8Error> for DirenvError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        DirenvError::FromUtf8(error)
    }
}
fn run_direnv(cmd_args: &[&str]) -> Result<std::process::Output> {
    let output = Command::new("direnv")
        .args(cmd_args)
        .output()
        .map_err(|e| DirenvError::Io(e))?;
    if !output.status.success() {
        return Err(DirenvError::CommandFailed {
            status: output.status,
            stderr: String::from_utf8(output.stderr)?,
        });
    }
    Ok(output)
}
fn path_to_str(path: &Path) -> Result<&str> {
    Ok(path.to_str().ok_or(DirenvError::InvalidPath)?)
}

impl Direnv {
    pub fn allow(path: &Path) -> Result<()> {
        run_direnv(&["allow", path_to_str(path)?])?;
        Ok(())
    }
    pub fn block(path: &Path) -> Result<()> {
        run_direnv(&["block", path_to_str(path)?])?;
        Ok(())
    }
    pub fn deny(path: &Path) -> Result<()> {
        run_direnv(&["deny", path_to_str(path)?])?;
        Ok(())
    }
    pub fn edit(path: &Path) -> Result<()> {
        run_direnv(&["edit", path_to_str(path)?])?;
        Ok(())
    }
    pub fn exec(path: &Path, cmd_args: &[&str]) -> Result<String> {
        let mut args = vec!["exec", path_to_str(path)?];
        args.extend_from_slice(cmd_args);
        let output = run_direnv(&args)?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn export(shell: &str) -> Result<()> {
        run_direnv(&["export", shell])?;
        Ok(())
    }
    pub fn fetchurl(url: &str, hash: &str) -> Result<String> {
        let output = run_direnv(&["fetchurl", url, hash])?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn grant(path: &Path) -> Result<()> {
        run_direnv(&["edit", path_to_str(path)?])?;
        Ok(())
    }
    pub fn hook(shell: &str) -> Result<String> {
        let output = run_direnv(&["hook", shell])?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn permit(path: &Path) -> Result<()> {
        run_direnv(&["permit", path_to_str(path)?])?;
        Ok(())
    }
    pub fn prune() -> Result<()> {
        run_direnv(&["prune"])?;
        Ok(())
    }
    pub fn reload() -> Result<()> {
        run_direnv(&["reload"])?;
        Ok(())
    }
    pub fn revoke(path: &Path) -> Result<()> {
        run_direnv(&["permit", path_to_str(path)?])?;
        Ok(())
    }
    pub fn status(cmd_args: &[&str]) -> Result<String> {
        let mut args = vec!["status"];
        args.extend_from_slice(cmd_args);
        let output = run_direnv(&args)?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn stdlib() -> Result<()> {
        run_direnv(&["stdlib"])?;
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
