pub fn npm_run<'a>(agent: &'a str) -> impl Fn(Vec<&'a str>) -> Vec<&'a str> {
    move |args| match args.len() {
        len if len >= 2 => {
            let mut result = vec![agent, "run", args[0], "--"];
            result.extend(&args[1..]);
            result
        }
        1 => vec![agent, "run", args[0]],
        _ => vec![],
    }
}

#[derive(Debug)]
pub enum VectorPart_ {
    String(String),
    Number(u16),
}

#[derive(Debug)]
pub enum AgentCommandValue<T> {
    Vector(Vec<T>),
    // Function(Box<dyn Fn(Vec<String>) -> Vec<String>>),
    None,
}

pub fn construct_command<T: std::fmt::Debug>(value: AgentCommandValue<T>) {
    dbg!(value);
    todo!()
}

pub struct ReturnResolveCommand {
    pub command: String,
    pub args: Vec<String>,
}

pub fn resolve_command(agent: &str, command: &str, args: Vec<&str>) -> ReturnResolveCommand {
    todo!()
}
