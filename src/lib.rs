//! Library to get os package manager or to know if system has specific commands

use std::{env, process::Command};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pm: &str = package_manager();
        assert!(!pm.is_empty())
    }

    #[test]
    fn existing_command() {
        let com: &str = "cd";
        assert!(check_command(com))
    }

    #[test]
    fn non_existing_command() {
        let com: &str = "uwu7";
        assert!(!check_command(com))
    }
}

/// Returns the system's package manager, if there is one.
/// Keep in mind this will only return the name of the package manager, 
/// so you can't just take the result of the function and use it with `install`, this is because not every pm installs or is used the same way.
/// # Example
/// ```
/// use pak_command::package_manager;
/// 
/// let pm: &str = package_manager();
/// println!("Your system's package manager is {}!", pm);
/// ```
pub fn package_manager() -> &'static str {
    let kernel: &str = env::consts::OS;
    match kernel {
        "windows" => return check_windows(),
        "freebsd" | "dragonfly" | "solaris" => return  "pkg",
        "openbsd" => return "pkg_add",
        "netbsd" => return "pkgsrc",
        "macos" => return check_mac(),
        "linux" => return check_linux(),
        _ => return "No package manager found."
    }
}


/// Returns if the given command name exists in current system.
/// If current system is Android or IOS it will always return false.
/// # Example
/// ```
/// use pak_command::check_command;
/// 
/// let first_command: &str = "cd";
/// let second_command: &str = "unexisting_command";
/// check_command(first_command); //true
/// check_command(second_command); //false
/// ```
pub fn check_command(command: &str) -> bool {
    let kernel: &str = env::consts::OS;
    match kernel {
        "android" | "ios" => return false,
        "windows" => return check_command_windows(command),
        _ => return check_command_unix(command)
    }
}

fn check_command_unix(command: &str) -> bool {
    Command::new("command")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn check_command_windows(command: &str) -> bool {
    Command::new("where")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn check_windows() -> &'static str  {
    if check_version_pm("winget") {
        return "winget";
    }
    return "No package manager found."
}

fn check_linux() -> &'static str {
    let packages = ["apt", "dnf", "zypper", "pacman", "emerge", "nix-env"];
    for pm in packages {
        if check_version_pm(pm) {
            return &pm;
        }
    }
    return "No package manager found.";
}

fn check_mac() -> &'static str {
    let packages = ["brew", "port", "fink"];
    for pm in packages {
        if check_version_pm(pm) {
            return &pm;
        }
    }
    return "No package manager found.";
}


fn check_version_pm(pm: &str) -> bool {
    if let Ok(output) = Command::new(pm).arg("--version").output() {
        if output.status.success() {
            return true;
        }
    }
    return false;
}