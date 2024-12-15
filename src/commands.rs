#[derive(Debug)]
pub struct Npm<'a> {
    pub agent: &'a str,
}
impl<'a> Npm<'a> {
    // TODO: add test
    pub fn run(&'a self, args: Vec<&'a str>) -> Vec<&'a str> {
        match args.len() {
            len if len >= 2 => {
                let mut result = vec![self.agent, "run", args[0], "--"];
                result.extend(&args[1..]);
                result
            }
            1 => vec![self.agent, "run", args[0]],
            _ => vec![],
        }
    }
}

#[derive(Debug)]
pub enum CommandList<'a> {
    Static(&'a [&'a str]),
    Dynamic(Npm<'a>),
    Empty,
}

pub static NPM: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["npm", "0"]),
    "run" => CommandList::Dynamic(Npm {agent: "npm"}),
    "install" => CommandList::Static(&["npm", "i", "0"]),
    "frozen" => CommandList::Static(&["npm", "ci"]),
    "global" => CommandList::Static(&["npm", "i", "-g", "0"]),
    "add" => CommandList::Static(&["npm", "i", "0"]),
    "upgrade" => CommandList::Static(&["npm", "update", "0"]),
    "upgrade-interactive" => CommandList::Empty,
    "execute" => CommandList::Static(&["npx", "0"]),
    "execute-local" => CommandList::Static(&["npx", "0"]),
    "uninstall" => CommandList::Static(&["npm", "uninstall", "0"]),
    "global_uninstall" => CommandList::Static(&["npm", "uninstall", "-g", "0"]),
};

// yarn 1
pub static YARN: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["yarn", "0"]),
    "run" => CommandList::Static(&["yarn", "run", "0"]),
    "install" => CommandList::Static(&["yarn", "install", "0"]),
    "frozen" => CommandList::Static(&["yarn", "install", "--frozen-lockfile"]),
    "global" => CommandList::Static(&["yarn", "global", "add", "0"]),
    "add" => CommandList::Static(&["yarn", "add", "0"]),
    "upgrade" => CommandList::Static(&["yarn", "upgrade", "0"]),
    "upgrade-interactive" => CommandList::Static(&["yarn", "upgrade-interactive", "0"]),
    "execute" => CommandList::Static(&["npx", "0"]),
    "execute-local" => CommandList::Static(&["yarn", "exec", "0"]),
    "uninstall" => CommandList::Static(&["yarn", "remove", "0"]),
    "global_uninstall" => CommandList::Static(&["yarn", "global", "remove", "0"]),
};

// yarn 2+
// Yarn 2+ removed 'global', see https://github.com/yarnpkg/berry/issues/821
pub static YARN_BERRY: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["yarn", "0"]),
    "run" => CommandList::Static(&["yarn", "run", "0"]),
    "install" => CommandList::Static(&["yarn", "install", "0"]),
    "frozen" => CommandList::Static(&["yarn", "install", "--immutable"]),
    "global" => CommandList::Static(&["yarn", "i", "-g", "0"]),
    "add" => CommandList::Static(&["yarn", "add", "0"]),
    "upgrade" => CommandList::Static(&["yarn", "up", "0"]),
    "upgrade-interactive" => CommandList::Static(&["yarn", "up", "-i", "0"]),
    "execute" => CommandList::Static(&["npx", "0"]),
    "execute-local" => CommandList::Static(&["yarn", "exec", "0"]),
    "uninstall" => CommandList::Static(&["yarn", "remove", "0"]),
    "global_uninstall" => CommandList::Static(&["yarn", "uninstall", "-g", "0"]),
};

pub static PNPM: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["pnpm", "0"]),
    "run" => CommandList::Static(&["pnpm", "run", "0"]),
    "install" => CommandList::Static(&["pnpm", "i", "0"]),
    "frozen" => CommandList::Static(&["pnpm", "i", "--frozen-lockfile"]),
    "global" => CommandList::Static(&["pnpm", "add", "-g", "0"]),
    "add" => CommandList::Static(&["pnpm", "add", "0"]),
    "upgrade" => CommandList::Static(&["pnpm", "update", "0"]),
    "upgrade-interactive" => CommandList::Static(&["pnpm", "update", "-i", "0"]),
    "execute" => CommandList::Static(&["pnpm", "dlx", "0"]),
    "execute-local" => CommandList::Static(&["pnpm", "exec", "0"]),
    "uninstall" => CommandList::Static(&["pnpm", "remove", "0"]),
    "global_uninstall" => CommandList::Static(&["pnpm", "remove", "--global", "0"]),
};

pub static PNPM_6: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["pnpm", "0"]),
    "run" => CommandList::Dynamic(Npm {agent: "pnpm"}),
    "install" => CommandList::Static(&["pnpm", "i", "0"]),
    "frozen" => CommandList::Static(&["pnpm", "i", "--frozen-lockfile"]),
    "global" => CommandList::Static(&["pnpm", "add", "-g", "0"]),
    "add" => CommandList::Static(&["pnpm", "add", "0"]),
    "upgrade" => CommandList::Static(&["pnpm", "update", "0"]),
    "upgrade-interactive" => CommandList::Static(&["pnpm", "update", "-i", "0"]),
    "execute" => CommandList::Static(&["pnpm", "dlx", "0"]),
    "execute-local" => CommandList::Static(&["pnpm", "exec", "0"]),
    "uninstall" => CommandList::Static(&["pnpm", "remove", "0"]),
    "global_uninstall" => CommandList::Static(&["pnpm", "remove", "--global", "0"]),
};

pub static BUN: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["bun", "0"]),
    "run" => CommandList::Static(&["bun", "run", "0"]),
    "install" => CommandList::Static(&["bun", "install", "0"]),
    "frozen" => CommandList::Static(&["bun", "install", "--frozen-lockfile"]),
    "global" => CommandList::Static(&["bun", "add", "-g", "0"]),
    "add" => CommandList::Static(&["bun", "add", "0"]),
    "upgrade" => CommandList::Static(&["bun", "update", "0"]),
    "upgrade-interactive" => CommandList::Static(&["bun", "update", "0"]),
    "execute" => CommandList::Static(&["bun", "x", "0"]),
    "execute-local" => CommandList::Static(&["bun", "x", "0"]),
    "uninstall" => CommandList::Static(&["bun", "remove", "0"]),
    "global_uninstall" => CommandList::Static(&["bun", "remove", "-g", "0"]),
};

pub static DENO: phf::Map<&'static str, CommandList> = phf::phf_map! {
    "agent" => CommandList::Static(&["deno", "0"]),
    "run" => CommandList::Static(&["deno", "run", "0"]),
    "install" => CommandList::Static(&["deno", "install", "0"]),
    "frozen" => CommandList::Static(&["deno", "install", "--frozen"]),
    "global" => CommandList::Static(&["deno", "install", "-g", "0"]),
    "add" => CommandList::Static(&["deno", "add", "0"]),
    "upgrade" => CommandList::Static(&["deno", "outdated", "--update", "0"]),
    "upgrade-interactive" => CommandList::Static(&["deno", "outdated", "--update", "0"]),
    "execute" => CommandList::Static(&["deno", "run", "0"]),
    "execute-local" => CommandList::Static(&["deno", "task", "--eval", "0"]),
    "uninstall" => CommandList::Static(&["deno", "remove", "0"]),
    "global_uninstall" => CommandList::Static(&["deno", "uninstall", "-g", "0"]),
};

pub static COMMANDS: phf::Map<&'static str, &'static phf::Map<&'static str, CommandList>> = phf::phf_map! {
    "npm" => &NPM,
    "yarn" => &YARN,
    "yarn@berry" => &YARN_BERRY,
    "pnpm" => &PNPM,
    // // pnpm v6.x or below
    "pnpm@6" => &PNPM_6,
    "bun" => &BUN,
    "deno" => &DENO,
};

#[derive(Debug, PartialEq)]
pub struct ResolveCommandReturn {
    pub command: String,
    pub args: Vec<String>,
}

// TODO: add test
pub fn resolve_command(
    agent: &str,
    command: &str,
    args: Vec<&str>,
) -> Option<ResolveCommandReturn> {
    let value = COMMANDS.get(agent).expect("COMMANDS.get(agent) error cuy");
    let value = value.get(command).expect("value.get(command) error cuy");
    construct_command(value, args)
}

// TODO: add test
pub fn construct_command(value: &CommandList, args: Vec<&str>) -> Option<ResolveCommandReturn> {
    match value {
        CommandList::Static(v) => {
            let list: Vec<_> = v
                .iter()
                .flat_map(|v| {
                    if *v == "0" {
                        return args.clone();
                    };
                    vec![v]
                })
                .collect();
            Some(ResolveCommandReturn {
                command: list.first().expect("list.get(0) error cuy!").to_string(),
                args: list[1..].iter().map(|&s| s.to_string()).collect(),
            })
        }
        CommandList::Dynamic(v) => {
            let list = v.run(args);
            Some(ResolveCommandReturn {
                command: list.first().expect("list.get(0) error cuy!").to_string(),
                args: list[1..].iter().map(|&s| s.to_string()).collect(),
            })
        }
        CommandList::Empty => None,
    }
}
