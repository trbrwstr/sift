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
fn analyze_nginx_filter_gt() {
    let output = sift(&["analyze", "examples/nginx.log", "--format", "nginx", "--filter", "status>400"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_nginx_filter_gte() {
    let output = sift(&["analyze", "examples/nginx.log", "--format", "nginx", "--filter", "status>=200"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_nginx_filter_lt() {
    let output = sift(&["analyze", "examples/nginx.log", "--format", "nginx", "--filter", "status<500"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_json_filter_level() {
    let output = sift(&["analyze", "examples/app.json", "--format", "json", "--filter", "level=ERROR"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_table_output() {
    let output = sift(&["analyze", "examples/app.json", "--format", "json", "--output", "table"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn analyze_json_output() {
    let output = sift(&["analyze", "examples/app.json", "--format", "json", "--output", "json"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("total"), "expected JSON with 'total' key, got: {}", stdout);
}

#[test]
fn analyze_top_flag() {
    let output = sift(&["analyze", "examples/nginx.log", "--format", "nginx", "--top", "3"]);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn nonexistent_file_exits_with_error() {
    let output = sift(&["stats", "examples/does_not_exist.log"]);
    assert!(!output.status.success());
}
