use semver::VersionReq;
use std::error::Error;
use std::fs;

mod package_lock_parser;
use package_lock_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let pkg_str = fs::read_to_string("test_fixtures/package-lock.small.json")?;
    let package_lock: PackageLock = serde_json::from_str(&pkg_str)?;

    let packages_versions = packages_to_packages_versions(&package_lock.packages);

    let result = package_version_exists(
        &packages_versions,
        "ajv",
        &VersionReq::parse(">=15").unwrap(),
    );

    dbg!(&result);

    Ok(())
    // TODO - bring in semver
    // https://github.com/dtolnay/semver#requirements
}
