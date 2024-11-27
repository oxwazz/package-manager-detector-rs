use package_manager_detector_rs::detect::*;
use std::{env, fs};

#[test]
fn it_works() {
    // Create a temporary directory for testing
    let temp_dir = env::temp_dir();
    
    // Create a nested directory structure
    let nested_dir = temp_dir.join("a/b/c");
    fs::create_dir_all(&nested_dir).expect("Failed to create nested directory");

    let result = lookup(Some(nested_dir.clone()));
    insta::assert_debug_snapshot!(result.collect::<Vec<_>>());
}
