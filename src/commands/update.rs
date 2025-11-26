use anyhow::{anyhow, Result};
use std::{process::Command, str::FromStr};

use crate::package::Packages;

/// Returns true if project needs to be rebuilt, false if it's already at the latest version
pub fn pull_repo(path: &str, package: &str) -> Result<bool> {
    let command = format!("cd {} && git pull origin main", path);
    let results = Command::new("sh").args(["-c", &command]).output()?;

    if !results.status.success() {
        let command = format!(
            "cd {} && git reset --hard main && git pull origin main",
            path
        );
        let results = Command::new("sh").args(["-c", &command]).output()?;

        if !results.status.success() {
            return Err(anyhow!(
                "Had problems updating {}: {}",
                package,
                String::from_utf8(results.stderr)?
            ));
        }
    };

    if String::from_utf8(results.stdout)?.contains("Already up to date.") {
        Ok(false)
    } else {
        Ok(true)
    }
}

fn update_all(force: bool) -> Result<()> {
    let packages = [Packages::Cs2Haskell, Packages::Lambdananas];

    for package in packages {
        if let Err(e) = package.update(force) {
            if package == Packages::Cs2Haskell {
                println!("{}", e);
            } else {
                return Err(e);
            }
        };
    }
    Ok(())
}

/// Does cleanup work, checks if there are files that shouldn't be there,
/// or should be moved and such.
/// Doesn't actually remove them for you, but suggests that they can be removed.
fn pre_update() -> Result<()> {
    Ok(())
}

pub fn handler(package: &Option<String>, force: bool) -> Result<()> {
    pre_update()?;

    if let Some(package_str) = package {
        let package = Packages::from_str(package_str)?;
        return package.update(force);
    }

    update_all(force)
}
