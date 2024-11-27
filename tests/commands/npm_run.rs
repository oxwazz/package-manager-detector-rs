use package_manager_detector_rs::commands::*;

/// npm_run
#[test]
fn test_npm_single_argument() {
    let npm = npm_run("npm");
    let output = npm(vec!["test"]);
    insta::assert_compact_debug_snapshot!(output);
}

#[test]
fn test_npm_multiple_arguments() {
    let npm = npm_run("npm");
    let output = npm(vec!["test", "unit", "--watch"]);
    insta::assert_compact_debug_snapshot!(output);
}

#[test]
fn test_yarn_single_argument() {
    let yarn = npm_run("yarn");
    let output = yarn(vec!["build"]);
    insta::assert_compact_debug_snapshot!(output);
}

#[test]
fn test_yarn_multiple_arguments() {
    let yarn = npm_run("yarn");
    let output = yarn(vec!["test", "integration"]);
    insta::assert_compact_debug_snapshot!(output);
}

#[test]
fn test_empty_args() {
    let npm = npm_run("npm");
    let output = npm(vec![]);
    insta::assert_compact_debug_snapshot!(output);
}
