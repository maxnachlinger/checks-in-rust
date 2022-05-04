use semver::VersionReq;
use std::error::Error;
use std::fs;

mod package_lock;
use package_lock::*;

fn main() -> Result<(), Box<dyn Error>> {
    let pkg_str = fs::read_to_string("test_fixtures/package-lock.small.json")?;
    let package_lock: PackageLock = serde_json::from_str(&pkg_str)?;

    let packages_versions = PackagesVersions::new(&package_lock.packages);

    let result = packages_versions.version_exists("ajv", &VersionReq::parse(">=15").unwrap());

    dbg!(&result);

    Ok(())
}
