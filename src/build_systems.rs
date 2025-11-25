use std::path::Path;

use anyhow::Result;

use crate::package::Packages;
use crate::shared;

enum BuildSystems {
    Default,
}

impl BuildSystems {
    fn build(&self) -> Result<Vec<String>> {
        self.clean()?;

        let build_system_output = match *self {
            Self::Default => {
                vec![1, 2, 3]
            }
        };

        shared::split_output(build_system_output)
    }

    fn clean(&self) -> Result<()> {
        match *self {
            _ => Ok(()),
        }
    }
}

pub fn verify_packages() -> bool {
    let packages = [Packages::Lambdananas];

    for package in packages {
        let mut found = false;

        for path in package.get_packages() {
            if Path::new(path).exists() {
                found = true;
            }
        }

        if !found {
            println!("Couldn't find {}", package);
            return false;
        }
    }
    true
}

pub fn find() -> Result<Vec<String>> {
    BuildSystems::Default.build()
}
