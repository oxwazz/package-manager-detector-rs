use package_manager_detector_rs::commands::{npm_run};

fn main() {
    println!("Hello, world!");
    let npm = npm_run("npm");
    let args = vec!["test", "unit"];
    let result = npm(args);
    dbg!(result);

    let yarn = npm_run("yarn");
    let single_arg = vec!["build"];
    let result2 = yarn(single_arg); // Returns: ["yarn", "run", "build"]
    dbg!(result2);
}
