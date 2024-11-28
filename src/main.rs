use dotenv::dotenv;
use package_manager_detector_rs::detect::detect;
use std::env;
use package_manager_detector_rs::commands::{CommandList, COMMANDS};

fn main() {
    dotenv().ok(); // Reads the .env file

    let api_key = env::var("API_KEY");
    match api_key {
        Ok(val) => println!("API_KEY: {:?}", val),
        Err(e) => println!("Error API_KEY: {}", e),
    }

    let tes = COMMANDS.get("deno").expect("ads1");
    let tes2 = tes.get("run").expect("ads2");
    match tes2 {
        CommandList::Static(v) => {
            dbg!(v.join(" "));
        }
        CommandList::Dynamic(v) => {
            dbg!(v.run(vec!["dev", "tes"]).join(" "));
        }
        CommandList::Empty => {
            todo!()
        }
    };

    // Try to get the value of the environment variable "npm_config_user_agent"
    // match env::var("npm_config_user_agent") {
    //     Ok(value) => {
    //         println!("npm_config_user_agent: {}", value);
    //     }
    //     Err(_) => {
    //         println!("npm_config_user_agent is not set");
    //     }
    // }

    // construct_command(AgentCommandValue::Vector(vec!["1213", "asdasd"]));
    // // EXPECTED
    // // const { command, args } = resolve_command(Pm.agent, 'add', ['@antfu/ni']) // { cli: 'pnpm', args: ['add', '@antfu/ni'] }
    // let agent = "npm";
    // let tes = resolve_command(agent, "add", vec!["@antfu/ni"]);
    // println!("{} {:?}", tes.command, tes.args);

    println!("{}", "tes");
    let tes = detect();
    println!("{:?}", tes);
}
