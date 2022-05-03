use std::error::Error;
use std::fs;

mod package_lock_parser;
use package_lock_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let pkg_str = fs::read_to_string("example/package-lock.json")?;
    let package_lock: PackageLock = serde_json::from_str(&pkg_str)?;

    let packages_versions = packages_to_packages_versions(&package_lock.packages);
    dbg!(&packages_versions);

    let found = &packages_versions.get("ajv");
    dbg!(&found);

    let not_found = &packages_versions.get("zzzzz");
    dbg!(&not_found);

    Ok(())
}
