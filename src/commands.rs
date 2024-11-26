pub fn npm_run<'a>(agent: &'a str) -> impl Fn(Vec<&'a str>) -> Vec<&'a str> {
    move |args| {
        match args.len() {
            len if len >= 2 => {
                let mut result  = vec![agent, "run", args[0], "--"];
                result.extend(&args[1..]);
                result
            },
            1 => vec![agent, "run", args[0]],
            _ => vec![]
        }
    }
}

#[derive(Debug)]
pub enum VectorPart_ {
    String(String),
    Number(u16)
}

#[derive(Debug)]
pub enum AgentCommandValue<T> {
    Vector(Vec<T>),
    // Function(Box<dyn Fn(Vec<String>) -> Vec<String>>),
    None,
}

pub fn construct_command<T: std::fmt::Debug>(value: AgentCommandValue<T>) {
    dbg!(value);
}

#[cfg(test)]
mod tests {
    use super::*;  // Import everything from parent module

    #[test]
    fn test_npm_single_argument() {
        let npm = npm_run("npm");
        let result = npm(vec!["test"]);
        assert_eq!(result, vec!["npm", "run", "test"]);
    }

    #[test]
    fn test_npm_multiple_arguments() {
        let npm = npm_run("npm");
        let result = npm(vec!["test", "unit", "--watch"]);
        assert_eq!(result, vec!["npm", "run", "test", "--", "unit", "--watch"]);
    }

    #[test]
    fn test_yarn_single_argument() {
        let yarn = npm_run("yarn");
        let result = yarn(vec!["build"]);
        assert_eq!(result, vec!["yarn", "run", "build"]);
    }

    #[test]
    fn test_yarn_multiple_arguments() {
        let yarn = npm_run("yarn");
        let result = yarn(vec!["test", "integration"]);
        assert_eq!(result, vec!["yarn", "run", "test", "--", "integration"]);
    }

    #[test]
    fn test_empty_args() {
        let npm = npm_run("npm");
        let result = npm(vec![]);
        assert!(result.is_empty());
    }
}


