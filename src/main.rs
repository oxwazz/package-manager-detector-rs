use package_manager_detector_rs::commands::{
    construct_command, npm_run, AgentCommandValue, VectorPart_,
};
use package_manager_detector_rs::detect::parse_package_json;
use std::env;
use std::path::Path;
use dotenv::dotenv;

struct ReturnResolveCommand {
    command: String,
    args: Vec<String>,
}

fn resolve_command(agent: &str, command: &str, args: Vec<&str>) -> ReturnResolveCommand {
    todo!()
}

fn main() {
    dotenv().ok(); // Reads the .env file

    let api_key = env::var("API_KEY");
    match api_key {
        Ok(val) => println!("API_KEY: {:?}", val),
        Err(e) => println!("Error API_KEY: {}", e),
    }
    println!("Hello, world!");
    let npm = npm_run("npm");
    let args = vec!["test", "unit"];
    let result = npm(args);
    dbg!(result);

    let yarn = npm_run("yarn");
    let single_arg = vec!["build"];
    let result2 = yarn(single_arg); // Returns: ["yarn", "run", "build"]
    dbg!(result2);

    // test
    let tes = parse_package_json(Path::new("package.json"), None).unwrap();
    dbg!(tes);
    // test

    // let tes = vec!["1213", 0];
    construct_command(AgentCommandValue::Vector(vec!["1213", "asdasd"]));

    // EXPECTED
    // const { command, args } = resolve_command(Pm.agent, 'add', ['@antfu/ni']) // { cli: 'pnpm', args: ['add', '@antfu/ni'] }
    let agent = "npm";
    let tes = resolve_command(agent, "add", vec!["@antfu/ni"]);
    println!("{} {:?}", tes.command, tes.args)
}
