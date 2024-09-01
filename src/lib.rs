use std::{env, process::Command};


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        println!("{}", package_manager())
    }
}

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