#[allow(unused_imports)]
use crate::detect::parse_package_json;
#[allow(unused_imports)]
use serde_json::json;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_yarn_v2_plus() {
    let content = json!({
        "packageManager": "^yarn@2.0.0"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(temp_file.path(), None);
    let package_info = result.unwrap();
    insta::assert_debug_snapshot!(package_info);
}

#[test]
fn test_yarn_v1() {
    let content = json!({
        "packageManager": "^yarn@1.22.19"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(temp_file.path(), None);
    let package_info = result.unwrap();
    insta::assert_debug_snapshot!(package_info);
}

#[test]
fn test_pnpm_v6() {
    let content = json!({
        "packageManager": "^pnpm@6.32.0"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(temp_file.path(), None);
    let package_info = result.unwrap();
    insta::assert_debug_snapshot!(package_info);
}

#[test]
fn test_pnpm_v7_plus() {
    let content = json!({
        "packageManager": "^pnpm@7.0.0"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(temp_file.path(), None);
    assert!(result.is_some());
    let package_info = result.unwrap();
    insta::assert_debug_snapshot!(package_info);
}

#[test]
fn test_other_known_agents() {
    let known_agents = ["npm", "bun"];
    for &agent in known_agents.iter() {
        let content = json!({
            "packageManager": format!("^{}@1.2.3", agent)
        });
        let temp_file = create_temp_json_file(content);
        let result = parse_package_json(temp_file.path(), None);
        let package_info = result.unwrap();
        insta::assert_debug_snapshot!(package_info);
    }
}

#[test]
fn test_unknown_agent_without_callback() {
    let content = json!({
        "packageManager": "^unknown@1.2.3"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(temp_file.path(), None);
    insta::assert_debug_snapshot!(result);
}

#[test]
fn test_unknown_agent_with_callback() {
    let mut called = false;
    let content = json!({
        "packageManager": "^unknown@1.2.3"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(
        temp_file.path(),
        Some(Box::new(|| {
            called = true;
        })),
    );
    assert!(called);
    insta::assert_debug_snapshot!(result);
}

#[test]
fn test_malformed_package_manager_string() {
    let content = json!({
        "packageManager": "malformed-string"
    });
    let temp_file = create_temp_json_file(content);
    let result = parse_package_json(temp_file.path(), None);
    insta::assert_debug_snapshot!(result);
}

// Helper function to create a temporary JSON file for testing
#[allow(dead_code)]
fn create_temp_json_file(content: serde_json::Value) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(
        temp_file,
        "{}",
        serde_json::to_string_pretty(&content).unwrap()
    )
    .unwrap();
    temp_file
}
