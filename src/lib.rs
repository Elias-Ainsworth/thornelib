mod commands;
mod direnv;

// use std::path::PathBuf;

// pub fn nix_in_path() -> bool {
//     use std::process::Command;
//     Command::new("which")
//         .arg("nix")
//         .output()
//         .map(|output| output.status.success())
//         .unwrap_or(false)
// }
//
// pub fn append_to_envrc(path: PathBuf, contents: Vec<&str>) -> std::io::Result<()> {
//     use std::{
//         fs::{self, OpenOptions},
//         io::Write,
//     };
//     let path: PathBuf = if path.as_path().ends_with(".envrc") {
//         path
//     } else {
//         path.as_path().join(".envrc")
//     };
//
//     let mut envfile = OpenOptions::new().append(true).open(&path)?;
//
//     for line in contents {
//         if fs::read_to_string(&path)?.contains(line) {
//             continue;
//         } else {
//             envfile.write(format!("{}\n", line).as_bytes())?;
//         }
//     }
//
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn use_flake() -> Result<(), std::io::Error> {
//         append_to_envrc(
//             PathBuf::from("./"),
//             vec!["use flake", "export EDITOR=nvim", "PATH_add target/debug"],
//         )?;
//         Ok(())
//     }
//
//     #[test]
//     fn check_nix() {
//         use std::{env, path::Path};
//         let on_nix_os = Path::new("/etc/nixos").exists();
//         let on_nix = Path::new("/nix").exists();
//         let in_nix_shell = env::var("IN_NIX_SHELL").is_ok();
//         let nix_in_path = nix_in_path();
//         assert!(
//             on_nix_os || on_nix || in_nix_shell || nix_in_path,
//             "You haven't installed Nix yet...? It's as if you're proud of the sin of ignorance."
//         );
//     }
// }
