extern crate serde;

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
) -> HashMap<String, Vec<String>> {
    packages
        .into_iter()
        .fold(HashMap::new(), |mut accum, (name, pkg)| {
            let key = clean_package_name(&name).to_owned();
            accum
                .entry(key)
                .or_insert_with(|| vec![])
                .push(pkg.version.clone());
            accum
        })
}
