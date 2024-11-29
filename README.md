# package_manager_detector_rs

[//]: # ([![npm version][npm-version-src]][npm-version-href])

[//]: # ([![npm downloads][npm-downloads-src]][npm-downloads-href])

[//]: # ([![JSDocs][jsdocs-src]][jsdocs-href])

[//]: # ([![License][license-src]][license-href])

Package manager detector is based on lock files and the `packageManager` field in the current project's `package.json`
file.

It will detect your `yarn.lock` / `pnpm-lock.yaml` / `package-lock.json` / `bun.lockb` / `deno.lock` to know the current
package manager and use the `packageManager` field in your `package.json` if present.

## Install

Run the following Cargo command in your project directory:

```sh
cargo add package_manager_detector_rs
```

Or add the following line to your Cargo.toml:

```toml
[dependencies]
package_manager_detector_rs = "0.2.5"
```

## Usage

To check the file system for which package manager is used:

```rust
use package_manager_detector_rs::detect::{detect};

fn main() {
    let pm = detect();
    if pm.is_none() {
        println!("Could not detect package manager")
    }

    println!("{:?}", pm);
    // HandlePackageManagerReturn { 
    //     name: "yarn", agent: "yarn@berry", version: "berry"
    // }
}
```

## Agents and Commands

This package includes package manager agents and their corresponding commands for:

- `'agent'` - run the package manager with no arguments
- `'install'` - install dependencies
- `'frozen'` - install dependencies using frozen lockfile
- `'add'` - add dependencies
- `'uninstall'` - remove dependencies
- `'global'` - install global packages
- `'global_uninstall'` - remove global packages
- `'upgrade'` - upgrade dependencies
- `'upgrade-interactive'` - upgrade dependencies interactively: not available for `npm` and `bun`
- `'execute'` - download & execute binary scripts
- `'execute-local'` - execute binary scripts (from package locally installed)
- `'run'` - run `package.json` scripts

### Using Agents and Commands

A `resolve_command` function is provided to resolve the command for a specific agent.

```rust
use package_manager_detector_rs::commands::{resolve_command};
use package_manager_detector_rs::detect::{detect};

fn main() {
    let pm = detect();
    if pm.is_none() {
        println!("Could not detect package manager")
    }

    println!("{:?}", pm);
    // HandlePackageManagerReturn {
    //     name: "yarn", agent: "yarn@berry", version: "berry"
    // }

    let agent = pm.unwrap().agent;
    let get_command = resolve_command(&agent, "add", vec!["@antfu/ni"]);
    let get_command = get_command.unwrap();

    println!("Detected the {} package manager. You can run a install with {} {}", &agent, get_command.command, get_command.args.join(" "))
    // Detected the yarn@berry package manager. You can run a install with yarn add @antfu/ni
}
```

You can check the source code for more information.

## Credit

package_manager_detector_rs is currently being developed and maintained
by [Muhammad Rahmahalim](https://github.com/oxwazz).<br>
This project is
like [package-manager-detector](https://github.com/antfu-collective/package-manager-detector) ([Anthony Fu](https://github.com/antfu))
but in rust.

## License

[MIT](./LICENSE) License © 2024-PRESENT [Muhammad Rahmahalim](https://github.com/oxwazz)

[//]: # (<!-- Badges -->)

[//]: # ()

[//]: # ([npm-version-src]: https://img.shields.io/npm/v/package-manager-detector?style=flat&colorA=18181B&colorB=F0DB4F)

[//]: # ([npm-version-href]: https://npmjs.com/package/package-manager-detector)

[//]: # ([npm-downloads-src]: https://img.shields.io/npm/dm/package-manager-detector?style=flat&colorA=18181B&colorB=F0DB4F)

[//]: # ([npm-downloads-href]: https://npmjs.com/package/package-manager-detector)

[//]: # ([jsdocs-src]: https://img.shields.io/badge/jsdocs-reference-080f12?style=flat&colorA=18181B&colorB=F0DB4F)

[//]: # ([jsdocs-href]: https://www.jsdocs.io/package/package-manager-detector)

[//]: # ([license-src]: https://img.shields.io/github/license/antfu-collective/package-manager-detector.svg?style=flat&colorA=18181B&colorB=F0DB4F)

[//]: # ([license-href]: https://github.com/antfu-collective/package-manager-detector/blob/main/LICENSE)
