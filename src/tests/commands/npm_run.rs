#[allow(unused_imports)]
use crate::commands::Npm;

/// npm_run
#[test]
fn test_npm_single_argument() {
    let npm = Npm { agent: "npm" };
    let output = npm.run(vec!["test"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_npm_multiple_arguments() {
    let npm = Npm { agent: "npm" };
    let output = npm.run(vec!["test", "unit", "--watch"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_yarn_single_argument() {
    let yarn = Npm { agent: "yarn" };
    let output = yarn.run(vec!["build"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_yarn_multiple_arguments() {
    let yarn = Npm { agent: "yarn" };
    let output = yarn.run(vec!["test", "integration"]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_empty_args() {
    let npm = Npm { agent: "yarn" };
    let output = npm.run(vec![]);
    insta::assert_debug_snapshot!(output);
}
