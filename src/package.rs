use std::fmt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use regex::Regex;
use reqwest::{blocking::Client, header::USER_AGENT};
use thiserror::Error;

use crate::commands::{
    shared::{get_final_path, get_temp_path, warn_path_var},
    update::pull_repo,
};

const LAMBDANANAS_RELEASE_API: &'static str =
    "https://api.github.com/repos/Epitech/lambdananas/releases/latest";
const LAMBDANANAS_RELEASE_LINK: &'static str =
    "https://github.com/Epitech/lambdananas/releases/download/$REPLACE/lambdananas";
const CS2_USER_AGENT: &'static str = "cs2-haskell <https://github.com/hugoschool/cs2-haskell>";

#[derive(Clone, Debug, PartialEq)]
pub enum Packages {
    Cs2Haskell,
    Lambdananas,
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

    #[error("Already installed, use cs2 update instead")]
    AlreadyInstalled,

    #[error(
        "{0} seems to be installed by a package manager, cs2 won't be able to install/update it"
    )]
    InstalledByPackageManager(String),
}

/// Release link must contain a $REPLACE
fn download_latest_release(
    release_url: &'static str,
    release_link: &'static str,
    temp_path: &str,
) -> Result<()> {
    let client = Client::new();
    let response = client
        .get(release_url)
        .header(USER_AGENT, CS2_USER_AGENT)
        .send()?
        .text()?;
    let mut tag_name: Option<&str> = None;

    if let Some((_, [tag])) = Regex::new(r#"tag_name":"(.*?)".*"#)?
        .captures_iter(&response)
        .map(|c| c.extract())
        .next()
    {
        tag_name = Some(tag);
    }

    let new_release_link = &release_link.replace("$REPLACE", tag_name.unwrap());
    if Command::new("wget")
        .args(["-O", temp_path, new_release_link])
        .status()?
        .success()
    {
        return Err(anyhow!("Impossible to download into {}", temp_path));
    };
    Ok(())
}

// Returns true if both files are the same, false if otherwise
fn compare_sha_sums(file: String) -> Result<bool> {
    Ok(true)
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
            Self::Lambdananas => "lambdananas",
        }
    }

    pub fn build(&self) -> Result<()> {
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
            Self::Lambdananas => {
                _ = download_latest_release(
                    LAMBDANANAS_RELEASE_API,
                    LAMBDANANAS_RELEASE_LINK,
                    &temp_path,
                );

                if !Command::new("chmod")
                    .args(["+x", &temp_path])
                    .status()?
                    .success()
                {
                    return Err(anyhow!("Impossible to chmod {}", temp_path));
                }
            }
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
