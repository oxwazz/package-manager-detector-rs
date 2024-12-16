#[allow(unused_imports)]
use crate::commands::DynamicCommand;

/// npm_run
#[test]
fn test_npm_single_argument() {
    let npm = DynamicCommand { agent: "npm" };
    let output = npm.run(vec!["test"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_npm_multiple_arguments() {
    let npm = DynamicCommand { agent: "npm" };
    let output = npm.run(vec!["test", "unit", "--watch"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_yarn_single_argument() {
    let yarn = DynamicCommand { agent: "yarn" };
    let output = yarn.run(vec!["build"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_yarn_multiple_arguments() {
    let yarn = DynamicCommand { agent: "yarn" };
    let output = yarn.run(vec!["test", "integration"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_empty_args() {
    let npm = DynamicCommand { agent: "yarn" };
    let output = npm.run(vec![]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn deno_run_single_argument() {
    let npm = DynamicCommand { agent: "deno" };
    let output = npm.run(vec!["test"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn deno_run_multiple_argument() {
    let npm = DynamicCommand { agent: "deno" };
    let output = npm.run(vec!["arg0", "arg1-0", "arg1-1"]);
    insta::assert_debug_snapshot!(output);
}
