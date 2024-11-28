use crate::constants::{AGENTS, LOCKS};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

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
    let pkg = match fs::File::open(filepath) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let pkg: Option<PackageJson> = serde_json::from_reader(pkg).ok();
    match pkg {
        None => None,
        Some(pkg_) => {
            let p = Regex::new(r"^\^").unwrap();
            let p = p.replace_all(pkg_.package_manager.as_str(), "");
            let p = p.split("@").collect::<Vec<&str>>();
            if p.len() < 2 {
                return None;
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

pub fn detect() -> Option<HandlePackageManagerReturn> {
    let directories = lookup(Some(PathBuf::from(".")));
    for directory in directories {
        // Look up for lock files
        for (lock_filename, lock_name) in LOCKS.into_iter() {
            let lockfile_path = directory.join(lock_filename);
            if lockfile_path.is_file() {
                let result = parse_package_json(lockfile_path.as_path(), None);
                match result {
                    None => {
                        return Some(HandlePackageManagerReturn {
                            name: lock_name.to_string(),
                            agent: lock_name.to_string(),
                            version: "".to_string(),
                        })
                    }
                    Some(v) => return Some(v),
                }
            }
        }

        // Look up for package.json
        let packager_path = directory.join("package.json");
        if packager_path.is_file() {
            return parse_package_json(packager_path.as_path(), None);
        }
    }

    None
}

/// I don't know how to implement this on rust,
/// because `process.env.npm_config_user_agent`
/// is node running process environment variable
pub fn get_user_agent() {
    todo!()
}
