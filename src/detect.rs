use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

const AGENTS: [&str; 7] = ["npm", "yarn", "yarn@berry", "pnpm", "pnpm@6", "bun", "deno"];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    package_manager: String,
}

#[derive(Debug)]
pub struct HandlePackageManagerReturn {
    name: String,
    agent: String,
    version: String,
}

pub fn handle_package_manager(
    filepath: &Path,
    on_unknown: Option<Box<dyn Fn()>>,
) -> Option<HandlePackageManagerReturn> {
    let pkg = fs::File::open(filepath).expect("package.json failed to open");
    let pkg: Option<PackageJson> = serde_json::from_reader(pkg).ok();
    match pkg {
        None => None,
        Some(pkg_) => {
            let p = Regex::new(r"^\^").unwrap();
            let p = p.replace_all(pkg_.package_manager.as_str(), "");
            let p = p.split("@").collect::<Vec<&str>>();
            let name = p[0];
            let version = p[1];
            let major_version = version.split(".").collect::<Vec<&str>>()[0]
                .parse::<i8>()
                .unwrap_or(0);

            if name == "yarn" && major_version > 1 {
                Some(HandlePackageManagerReturn {
                    name: name.to_string(),
                    agent: String::from("yarn@berry"),
                    version: String::from("berry"),
                })
            } else if name == "pnpm" && major_version < 7 {
                Some(HandlePackageManagerReturn {
                    name: name.to_string(),
                    agent: String::from("pnpm@6"),
                    version: version.to_string(),
                })
            } else if AGENTS.contains(&name) {
                Some(HandlePackageManagerReturn {
                    name: name.to_string(),
                    agent: name.to_string(),
                    version: version.to_string(),
                })
            } else {
                match on_unknown {
                    None => None,
                    Some(cb) => {
                        cb();
                        None
                    }
                }
            }
        }
    }
}
