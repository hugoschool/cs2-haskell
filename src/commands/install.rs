use std::str::FromStr;

use anyhow::Result;

use crate::{commands::shared::get_final_path, package::Packages, shared::create_directory};

fn install_all() -> Result<()> {
    let all_packages = [Packages::Lambdananas];

    for package in all_packages {
        package.install()?;
    }
    Ok(())
}

pub fn handler(package: &Option<String>) -> Result<()> {
    create_directory(get_final_path("").as_str())?;

    if let Some(package_str) = package {
        let package = Packages::from_str(package_str)?;
        return package.install();
    }

    install_all()
}
