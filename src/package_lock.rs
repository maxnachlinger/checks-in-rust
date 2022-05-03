extern crate serde;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub version: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PackageLock {
    pub packages: HashMap<String, Package>,
}

fn clean_package_name(name: &str) -> &str {
    name.split("node_modules/").into_iter().last().unwrap()
}

pub fn packages_to_packages_versions(
    packages: &HashMap<String, Package>,
    // using Vec<String> for versions since we don't get duplicates in package-lock.packages
    // if I'm wrong, we'll use a HashSet<String> :)
) -> HashMap<String, Vec<Version>> {
    packages
        .into_iter()
        .fold(HashMap::new(), |mut accum, (name, pkg)| {
            let key = clean_package_name(&name).to_owned();
            accum
                .entry(key)
                .or_insert_with(|| vec![])
                .push(Version::parse(&pkg.version).unwrap());
            accum
        })
}

pub fn package_version_exists(
    packages_versions: &HashMap<String, Vec<Version>>,
    package_name: &str,
    version_requirement: &VersionReq,
) -> bool {
    packages_versions
        .get(package_name)
        .map(|value| {
            value
                .iter()
                .any(|version| version_requirement.matches(version))
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs;

    #[test]
    fn package_version_exists_works() -> Result<(), Box<dyn Error>> {
        let pkg_str = fs::read_to_string("test_fixtures/package-lock.minimal.json")?;
        let package_lock: PackageLock = serde_json::from_str(&pkg_str)?;
        let packages_versions = packages_to_packages_versions(&package_lock.packages);

        assert!(package_version_exists(
            &packages_versions,
            "lodash",
            &VersionReq::parse(">=4").unwrap()
        ));
        assert!(!package_version_exists(
            &packages_versions,
            "lodash",
            &VersionReq::parse(">=99999").unwrap()
        ));
        assert!(!package_version_exists(
            &packages_versions,
            "zzzzzz",
            &VersionReq::parse(">=0").unwrap()
        ));
        Ok(())
    }

    #[test]
    fn packages_to_packages_versions_works() -> Result<(), Box<dyn Error>> {
        let pkg_str = fs::read_to_string("test_fixtures/package-lock.minimal.json")?;
        let package_lock: PackageLock = serde_json::from_str(&pkg_str)?;
        let packages_versions = packages_to_packages_versions(&package_lock.packages);

        assert!(packages_versions.contains_key("lodash"));
        assert_eq!(
            packages_versions
                .get("lodash")
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            "4.17.21",
        );

        assert!(packages_versions.contains_key(""));
        assert_eq!(
            packages_versions
                .get("")
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            "1.0.0",
        );
        Ok(())
    }

    #[test]
    fn clean_package_name_works() {
        let package_name = "node_modules/light-my-request/node_modules/ajv";
        let package_name_cleaned = clean_package_name(package_name);
        assert_eq!(package_name_cleaned, "ajv");
    }

    #[test]
    fn clean_package_name_works_with_namespaced_packages() {
        let package_name = "node_modules/light-my-request/node_modules/@walmart/test";
        let package_name_cleaned = clean_package_name(package_name);
        assert_eq!(package_name_cleaned, "@walmart/test");
    }
}
