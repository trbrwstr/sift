use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn sift(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "-p", "sift-cli", "--"])
        .args(args)
        .current_dir(workspace_root())
        .output()
        .expect("failed to run sift")
}

#[test]
fn stats_nginx_succeeds() {
    let output = sift(&["stats", "examples/nginx.log", "--format", "nginx"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn stats_json_succeeds() {
    let output = sift(&["stats", "examples/app.json", "--format", "json"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_nginx_with_filter() {
    let output = sift(&["analyze", "examples/nginx.log", "--format", "nginx", "--filter", "status>400"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_table_output() {
    let output = sift(&["analyze", "examples/app.json", "--format", "json", "--output", "table"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn nonexistent_file_exits_with_error() {
    let output = sift(&["stats", "examples/does_not_exist.log"]);
    assert!(!output.status.success());
}
