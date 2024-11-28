# package-manager-detector-rs

[//]: # ([![npm version][npm-version-src]][npm-version-href])

[//]: # ([![npm downloads][npm-downloads-src]][npm-downloads-href])

[//]: # ([![JSDocs][jsdocs-src]][jsdocs-href])

[//]: # ([![License][license-src]][license-href])

Package manager detector is based on lock files and the `packageManager` field in the current project's `package.json`
file.

It will detect your `yarn.lock` / `pnpm-lock.yaml` / `package-lock.json` / `bun.lockb` / `deno.lock` to know the current
package manager and use the `packageManager` field in your `package.json` if present.

## Install

WIP

[//]: # (```sh)

[//]: # (# pnpm)

[//]: # (pnpm add package-manager-detector)

[//]: # ()

[//]: # (# npm)

[//]: # (npm i package-manager-detector)

[//]: # ()

[//]: # (# yarn)

[//]: # (yarn add package-manager-detector)

[//]: # (```)

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

WIP

[//]: # (This package includes package manager agents and their corresponding commands for:)

[//]: # ()

[//]: # (- `'agent'` - run the package manager with no arguments)

[//]: # (- `'install'` - install dependencies)

[//]: # (- `'frozen'` - install dependencies using frozen lockfile)

[//]: # (- `'add'` - add dependencies)

[//]: # (- `'uninstall'` - remove dependencies)

[//]: # (- `'global'` - install global packages)

[//]: # (- `'global_uninstall'` - remove global packages)

[//]: # (- `'upgrade'` - upgrade dependencies)

[//]: # (- `'upgrade-interactive'` - upgrade dependencies interactively: not available for `npm` and `bun`)

[//]: # (- `'execute'` - download & execute binary scripts)

[//]: # (- `'execute-local'` - execute binary scripts &#40;from package locally installed&#41;)

[//]: # (- `'run'` - run `package.json` scripts)

[//]: # ()

[//]: # (### Using Agents and Commands)

[//]: # ()

[//]: # (A `resolve_command` function is provided to resolve the command for a specific agent.)

[//]: # ()

[//]: # (```ts)

[//]: # (import { resolve_command } from 'package-manager-detector/commands')

[//]: # (import { detect } from 'package-manager-detector/detect')

[//]: # ()

[//]: # (const pm = await detect&#40;&#41;)

[//]: # (if &#40;!pm&#41;)

[//]: # (  throw new Error&#40;'Could not detect package manager'&#41;)

[//]: # ()

[//]: # (const { command, args } = resolve_command&#40;pm.agent, 'add', ['@antfu/ni']&#41; // { command: 'pnpm', args: ['add', '@antfu/ni'] })

[//]: # (console.log&#40;`Detected the ${pm.agent} package manager. You can run a install with ${command} ${args.join&#40;' '&#41;}`&#41;)

[//]: # (```)

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
