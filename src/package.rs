use std::fmt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use anyhow::{Ok, Result, anyhow};
use thiserror::Error;

use crate::commands::{
    shared::{get_final_path, get_temp_path, warn_path_var},
    update::pull_repo,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Packages {
    Cs2Haskell,
    Lambdananas
}

#[derive(Error, Debug)]
enum PackagesError {
    #[error("Impossible to build {0}")]
    Build(Packages),

    #[error("Impossible to install {0}")]
    Install(Packages),

    #[error("Impossible to move {0} to it's destination")]
    Move(Packages),

    #[error("Impossible to find {0}, are you sure it is installed?")]
    NotFound(Packages),

    #[error("Impossible to clone {0}, make sure you have the permissions to do so")]
    RepoClone(String),

    #[error("Already installed, use cs2 update instead")]
    AlreadyInstalled,

    #[error(
        "{0} seems to be installed by a package manager, cs2 won't be able to install/update it"
    )]
    InstalledByPackageManager(String),
}

fn clone_repo(link: &str, temp_path: &str) -> Result<()> {
    if !Command::new("git")
        .args(["clone", link, temp_path])
        .status()?
        .success()
    {
        return Err(PackagesError::RepoClone(link.to_string()).into());
    };

    Ok(())
}

fn move_to_final_path(temp_path: &str, final_path: &Path) -> Result<()> {
    let final_path_str = final_path.to_str().unwrap();

    if final_path.exists() {
        return Ok(());
    }

    if !Command::new("sudo")
        .args(["mv", temp_path, final_path_str])
        .status()?
        .success()
    {
        return Err(anyhow!("Impossible to move to {}", final_path_str));
    };
    Ok(())
}

impl FromStr for Packages {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        match input.to_ascii_lowercase().as_str() {
            "cs2-haskell" => Ok(Self::Cs2Haskell),
            "lambdananas" => Ok(Self::Lambdananas),
            _ => Err(anyhow!("Couldn't find package")),
        }
    }
}

impl Packages {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Cs2Haskell => "cs2-haskell",
            Self::Lambdananas => "lambdananas"
        }
    }

    pub fn build(&self, ) -> Result<()> {
        match *self {
            Self::Cs2Haskell => {
                let build_command = format!("cd {} && ./compile.sh", get_final_path(self.as_str()));

                if !Command::new("sh")
                    .args(["-c", build_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(PackagesError::Build(Self::Cs2Haskell).into());
                }
            }
            Self::Lambdananas => {}
        }
        Ok(())
    }

    pub fn get_packages(&self) -> &[&str] {
        match *self {
            _ => &[],
        }
    }

    pub fn verify_install(&self) -> Result<()> {
        let packages = self.get_packages();
        let final_path = get_final_path(self.as_str());

        for package in packages {
            if Path::new(package).exists() && !Path::new(&final_path).exists() {
                return Err(PackagesError::InstalledByPackageManager(package.to_string()).into());
            }
        }
        Ok(())
    }

    pub fn install(&self) -> Result<()> {
        let package = self.as_str();
        let temp_path = get_temp_path(package);
        let final_path = get_final_path(package);

        self.verify_install()?;

        if Path::new(&final_path).exists() {
            return Err(PackagesError::AlreadyInstalled.into());
        }

        println!("Installing {}", package);

        match *self {
            _ => {}
        }

        self.build()?;
        _ = warn_path_var("/usr/local/bin");

        Ok(())
    }

    pub fn update(&self, force: bool) -> Result<()> {
        let package = self.as_str();
        let path = get_final_path(package);

        self.verify_install()?;

        if !Path::new(&path).exists() {
            return Err(PackagesError::NotFound(self.clone()).into());
        }

        println!("Updating {}", package);

        if pull_repo(&path, self.as_str())? || force {
            self.build()?;
        } else {
            println!("Nothing to update");
        }

        Ok(())
    }
}

impl fmt::Display for Packages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
