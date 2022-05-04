use semver::VersionReq;
use std::error::Error;
use std::path::Path;

mod package_lock;
use package_lock::*;

fn main() -> Result<(), Box<dyn Error>> {
    let packages_versions =
        PackagesVersions::new(Path::new("test_fixtures/package-lock.small.json"));

    let v6_exists =
        packages_versions.package_version_exists("ajv", &VersionReq::parse("6.12.6").unwrap());
    dbg!(&v6_exists);

    let v15_or_greater_exists =
        packages_versions.package_version_exists("ajv", &VersionReq::parse(">=15").unwrap());
    dbg!(&v15_or_greater_exists);

    Ok(())
}
