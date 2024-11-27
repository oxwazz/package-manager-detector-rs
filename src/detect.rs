use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const AGENTS: [&str; 7] = ["npm", "yarn", "yarn@berry", "pnpm", "pnpm@6", "bun", "deno"];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub package_manager: String,
}

#[derive(Debug)]
pub struct HandlePackageManagerReturn {
    pub name: String,
    pub agent: String,
    pub version: String,
}

pub fn parse_package_json<'a>(
    filepath: &Path,
    on_unknown: Option<Box<dyn FnMut() + 'a>>,
) -> Option<HandlePackageManagerReturn> {
    let pkg = fs::File::open(filepath).expect("package.json failed to open");
    let pkg: Option<PackageJson> = serde_json::from_reader(pkg).ok();
    match pkg {
        None => None,
        Some(pkg_) => {
            let p = Regex::new(r"^\^").unwrap();
            let p = p.replace_all(pkg_.package_manager.as_str(), "");
            let p = p.split("@").collect::<Vec<&str>>();
            if p.len() < 2 {
                return None; // Or handle the error as appropriate
            }
            let name = p[0];
            let version = p[1];
            let major_version = version.split(".").collect::<Vec<&str>>()[0]
                .parse::<u8>()
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
                    Some(mut cb) => {
                        cb();
                        None
                    }
                }
            }
        }
    }
}

pub fn lookup(cwd: Option<PathBuf>) -> impl Iterator<Item = PathBuf> {
    // Default to current working directory if not provided
    let directory = cwd.unwrap_or_else(|| std::env::current_dir().unwrap_or(PathBuf::from(".")));

    // Canonicalize the path to resolve any symlinks or relative paths
    let mut directory = dunce::canonicalize(directory).unwrap();
    let root = directory.ancestors().last().unwrap().to_path_buf();

    std::iter::from_fn(move || {
        // If we have reached the root, stop iteration
        if directory == root {
            return None;
        }

        // Store the current directory to return
        let current = directory.clone();

        // Move up to the parent directory, or stop at the root
        directory = directory.parent().unwrap_or(&root).to_path_buf();

        Some(current)
    })
}
