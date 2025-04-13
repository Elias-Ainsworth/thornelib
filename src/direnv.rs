use std::{
    any::Any,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    result, str,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Canonicalization Error: {0}")]
pub struct CanonicalizeError(#[from] std::io::Error);

#[derive(Error, Debug)]
pub enum DirenvError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("From UTF-8 Error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Command failed with status {status:?}: stderr")]
    CommandFailed {
        status: std::process::ExitStatus,
        stderr: String,
    },
    #[error("Invalid path (non-UTF-8)")]
    InvalidPath,
    #[error(transparent)]
    CanonicalizeError(#[from] CanonicalizeError),
}

pub type Result<T> = result::Result<T, DirenvError>;

pub trait DirenvRunner: Any {
    fn run_direnv(&self, cmd_args: &[String]) -> Result<Output>;
    fn as_any(&self) -> &dyn Any;
}

pub struct RealDirenv;

impl DirenvRunner for RealDirenv {
    fn run_direnv(&self, cmd_args: &[String]) -> Result<Output> {
        let output = Command::new("direnv").args(cmd_args).output()?;
        if !output.status.success() {
            return Err(DirenvError::CommandFailed {
                status: output.status,
                stderr: String::from_utf8(output.stderr)?,
            });
        }
        Ok(output)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn canonicalize_path(path: &Path) -> result::Result<PathBuf, CanonicalizeError> {
    Ok(fs::canonicalize(path)?)
}

fn path_to_str(path: &Path) -> Result<String> {
    Ok(canonicalize_path(path)?
        .as_path()
        .to_str()
        .ok_or(DirenvError::InvalidPath)?
        .to_owned())
}

pub struct Direnv {
    runner: Box<dyn DirenvRunner>,
}

impl Direnv {
    pub fn new() -> Self {
        Direnv {
            runner: Box::new(RealDirenv),
        }
    }
    pub fn with_runner(runner: Box<dyn DirenvRunner>) -> Self {
        Direnv { runner }
    }
    pub fn allow(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["allow".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn block(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["block".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn deny(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["deny".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn edit(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["edit".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn exec(&self, path: &Path, cmd_args: &[String]) -> Result<String> {
        let canonicalized_str = path_to_str(path)?;
        let mut args = vec!["exec".to_string(), canonicalized_str];
        args.extend_from_slice(cmd_args);
        let output = self.runner.run_direnv(&args)?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn export(&self, shell: String) -> Result<()> {
        self.runner.run_direnv(&["export".to_string(), shell])?;
        Ok(())
    }
    pub fn fetchurl(&self, url: String, hash: String) -> Result<String> {
        let output = self
            .runner
            .run_direnv(&["fetchurl".to_string(), url, hash])?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn grant(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["grant".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn hook(&self, shell: String) -> Result<String> {
        let output = self.runner.run_direnv(&["hook".to_string(), shell])?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn permit(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["permit".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn prune(&self) -> Result<()> {
        self.runner.run_direnv(&["prune".to_string()])?;
        Ok(())
    }
    pub fn reload(&self) -> Result<()> {
        self.runner.run_direnv(&["reload".to_string()])?;
        Ok(())
    }
    pub fn revoke(&self, path: &Path) -> Result<()> {
        let canonicalized_str = path_to_str(path)?;
        self.runner
            .run_direnv(&["revoke".to_string(), canonicalized_str])?;
        Ok(())
    }
    pub fn status(&self, cmd_args: &[String]) -> Result<String> {
        let mut args = vec!["status".to_string()];
        args.extend_from_slice(cmd_args);
        let output = self.runner.run_direnv(&args)?;
        Ok(String::from_utf8(output.stdout)?)
    }
    pub fn stdlib(&self) -> Result<()> {
        self.runner.run_direnv(&["stdlib".to_string()])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;
    use std::cell::RefCell;
    use std::os::unix::process::ExitStatusExt;

    struct MockRunner {
        pub called_with: RefCell<Vec<Vec<String>>>,
    }
    impl MockRunner {
        fn new() -> Self {
            MockRunner {
                called_with: RefCell::new(vec![]),
            }
        }
        fn get_calls(&self) -> Vec<Vec<String>> {
            self.called_with.borrow().clone()
        }
    }

    impl DirenvRunner for MockRunner {
        fn run_direnv(&self, cmd_args: &[String]) -> Result<Output> {
            self.called_with.borrow_mut().push(cmd_args.to_vec());

            Ok(Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: b"mock output".to_vec(),
                stderr: vec![],
            })
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn direnv_stdlib_ok() -> Result<()> {
        let direnv = Direnv::with_runner(Box::new(MockRunner::new()));
        direnv.stdlib()?;

        let calls = direnv
            .runner
            .as_any()
            .downcast_ref::<MockRunner>()
            .expect("expected MockRunner")
            .get_calls();
        assert_eq!(calls[0], vec!["stdlib".to_string()]);
        Ok(())
    }
}
